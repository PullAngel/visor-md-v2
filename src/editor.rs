//! Modelo de edición de fuente, independiente de parser, layout y ventana.
//!
//! Markdown se modifica como texto UTF-8. No se vuelve a serializar el AST:
//! así una construcción que el lector aún no conoce conserva sus bytes cuando
//! una edición local ocurre a su alrededor.

use std::collections::VecDeque;
use std::ops::Range;

use ropey::Rope;

/// Fuente editable almacenada como rope. La API pública conserva offsets de
/// bytes UTF-8 porque el parser, los rangos Markdown y el guardado ya usan esa
/// unidad; la conversión a índices de carácter queda encapsulada aquí.
#[derive(Clone, Debug, Default)]
pub struct TextBuffer {
    rope: Rope,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_text(source: &str) -> Self {
        Self {
            rope: Rope::from_str(source),
        }
    }

    pub fn len_bytes(&self) -> usize {
        self.rope.len_bytes()
    }

    pub fn is_empty(&self) -> bool {
        self.rope.len_bytes() == 0
    }

    pub fn is_char_boundary(&self, byte: usize) -> bool {
        if byte > self.len_bytes() {
            return false;
        }
        self.rope
            .try_byte_to_char(byte)
            .is_ok_and(|character| self.rope.char_to_byte(character) == byte)
    }

    pub fn slice_bytes(&self, range: Range<usize>) -> Result<String, EditError> {
        if range.start > range.end
            || !self.is_char_boundary(range.start)
            || !self.is_char_boundary(range.end)
        {
            return Err(EditError::InvalidRange);
        }
        let start = self.rope.byte_to_char(range.start);
        let end = self.rope.byte_to_char(range.end);
        Ok(self.rope.slice(start..end).to_string())
    }

    fn replace_range(&mut self, range: Range<usize>, inserted: &str) -> Result<(), EditError> {
        if range.start > range.end
            || !self.is_char_boundary(range.start)
            || !self.is_char_boundary(range.end)
        {
            return Err(EditError::InvalidRange);
        }
        let start = self.rope.byte_to_char(range.start);
        let end = self.rope.byte_to_char(range.end);
        self.rope.remove(start..end);
        self.rope.insert(start, inserted);
        Ok(())
    }

    fn previous_boundary(&self, byte: usize) -> usize {
        let character = self.rope.byte_to_char(byte);
        self.rope.char_to_byte(character.saturating_sub(1))
    }

    fn next_boundary(&self, byte: usize) -> usize {
        let character = self.rope.byte_to_char(byte);
        self.rope
            .char_to_byte((character + 1).min(self.rope.len_chars()))
    }

    fn line_start(&self, byte: usize) -> usize {
        self.rope.line_to_byte(self.rope.byte_to_line(byte))
    }

    fn line_end(&self, line: usize) -> usize {
        let start = self.rope.line_to_byte(line);
        let slice = self.rope.line(line);
        let mut end = start + slice.len_bytes();
        if slice.len_chars() > 0 && slice.char(slice.len_chars() - 1) == '\n' {
            end -= 1;
        }
        end
    }

    pub fn lines(&self) -> ropey::iter::Lines<'_> {
        self.rope.lines()
    }
}

impl std::fmt::Display for TextBuffer {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for chunk in self.rope.chunks() {
            formatter.write_str(chunk)?;
        }
        Ok(())
    }
}

/// Límite de memoria del historial por documento. Al alcanzarlo se descartan
/// los cambios más antiguos, nunca texto actual ni cambios no guardados.
pub const MAX_HISTORY_BYTES: usize = 4 * 1024 * 1024;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Change {
    start: usize,
    removed: String,
    inserted: String,
    before_revision: u64,
    after_revision: u64,
}

impl Change {
    fn bytes(&self) -> usize {
        self.removed.len() + self.inserted.len()
    }
}

/// Error de edición que evita partir una secuencia UTF-8 o aplicar historial a
/// un documento que ya no representa el estado esperado.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EditError {
    InvalidRange,
    InconsistentHistory,
}

/// Historial reversible para una fuente UTF-8. No retiene una copia completa
/// del documento: memoria proporcional a cambios, con límite explícito.
#[derive(Debug)]
pub struct EditHistory {
    undo: VecDeque<Change>,
    redo: Vec<Change>,
    history_bytes: usize,
    next_revision: u64,
    current_revision: u64,
    saved_revision: u64,
}

impl Default for EditHistory {
    fn default() -> Self {
        Self {
            undo: VecDeque::new(),
            redo: Vec::new(),
            history_bytes: 0,
            next_revision: 1,
            current_revision: 0,
            saved_revision: 0,
        }
    }
}

impl EditHistory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_dirty(&self) -> bool {
        self.current_revision != self.saved_revision
    }

    pub fn current_revision(&self) -> u64 {
        self.current_revision
    }

    pub fn mark_saved(&mut self) {
        self.saved_revision = self.current_revision;
    }

    pub fn mark_recovered(&mut self) {
        self.current_revision = self.next_revision;
        self.next_revision = self.next_revision.saturating_add(1);
        self.saved_revision = 0;
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    /// Reemplaza un rango de bytes que debe coincidir con límites de carácter.
    /// Devuelve `false` para una operación nula, que no ensucia el documento.
    pub fn apply(
        &mut self,
        source: &mut TextBuffer,
        range: Range<usize>,
        inserted: &str,
    ) -> Result<bool, EditError> {
        if range.start > range.end
            || range.end > source.len_bytes()
            || !source.is_char_boundary(range.start)
            || !source.is_char_boundary(range.end)
        {
            return Err(EditError::InvalidRange);
        }

        let removed = source.slice_bytes(range.clone())?;
        if removed == inserted {
            return Ok(false);
        }
        self.clear_redo();
        let before_revision = self.current_revision;
        let after_revision = self.next_revision;
        self.next_revision = self.next_revision.saturating_add(1);
        source.replace_range(range.clone(), inserted)?;
        let change = Change {
            start: range.start,
            removed,
            inserted: inserted.to_owned(),
            before_revision,
            after_revision,
        };
        self.current_revision = after_revision;
        self.push_new_undo(change);
        Ok(true)
    }

    pub fn undo(&mut self, source: &mut TextBuffer) -> Result<bool, EditError> {
        let Some(change) = self.undo.pop_back() else {
            return Ok(false);
        };
        if self.current_revision != change.after_revision {
            self.undo.push_back(change);
            return Err(EditError::InconsistentHistory);
        }
        let end = change.start.saturating_add(change.inserted.len());
        if end > source.len_bytes()
            || !source.is_char_boundary(change.start)
            || !source.is_char_boundary(end)
            || source.slice_bytes(change.start..end)?.as_str() != change.inserted
        {
            self.undo.push_back(change);
            return Err(EditError::InconsistentHistory);
        }
        source.replace_range(change.start..end, &change.removed)?;
        self.current_revision = change.before_revision;
        self.redo.push(change);
        Ok(true)
    }

    pub fn redo(&mut self, source: &mut TextBuffer) -> Result<bool, EditError> {
        let Some(change) = self.redo.pop() else {
            return Ok(false);
        };
        if self.current_revision != change.before_revision {
            self.redo.push(change);
            return Err(EditError::InconsistentHistory);
        }
        let end = change.start.saturating_add(change.removed.len());
        if end > source.len_bytes()
            || !source.is_char_boundary(change.start)
            || !source.is_char_boundary(end)
            || source.slice_bytes(change.start..end)?.as_str() != change.removed
        {
            self.redo.push(change);
            return Err(EditError::InconsistentHistory);
        }
        source.replace_range(change.start..end, &change.inserted)?;
        self.current_revision = change.after_revision;
        // El cambio ya está contabilizado mientras estuvo en `redo`; moverlo
        // de vuelta no puede cobrar memoria dos veces ni expulsar undo válido.
        self.undo.push_back(change);
        Ok(true)
    }

    fn clear_redo(&mut self) {
        for change in self.redo.drain(..) {
            self.history_bytes = self.history_bytes.saturating_sub(change.bytes());
        }
    }

    fn push_new_undo(&mut self, change: Change) {
        self.history_bytes = self.history_bytes.saturating_add(change.bytes());
        self.undo.push_back(change);
        while self.history_bytes > MAX_HISTORY_BYTES {
            let Some(oldest) = self.undo.pop_front() else {
                break;
            };
            self.history_bytes = self.history_bytes.saturating_sub(oldest.bytes());
        }
    }
}

/// Estado de interacción para un editor de fuente. Los offsets siempre caen en
/// límites de caracteres UTF-8; el renderer puede traducirlos después a líneas
/// y píxeles sin convertirse en autoridad sobre el texto.
#[derive(Debug, Default)]
pub struct SourceEditor {
    history: EditHistory,
    cursor: usize,
    anchor: usize,
    preferred_column: Option<usize>,
}

impl SourceEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn anchor(&self) -> usize {
        self.anchor
    }

    pub fn selection(&self) -> Range<usize> {
        self.cursor.min(self.anchor)..self.cursor.max(self.anchor)
    }

    pub fn is_dirty(&self) -> bool {
        self.history.is_dirty()
    }

    pub fn revision(&self) -> u64 {
        self.history.current_revision()
    }

    pub fn mark_saved(&mut self) {
        self.history.mark_saved();
    }

    pub fn mark_recovered(&mut self) {
        self.history.mark_recovered();
    }

    pub fn select_all(&mut self, source: &TextBuffer) {
        self.anchor = 0;
        self.cursor = source.len_bytes();
        self.preferred_column = None;
    }

    pub fn set_cursor(
        &mut self,
        source: &TextBuffer,
        offset: usize,
        extend: bool,
    ) -> Result<(), EditError> {
        if offset > source.len_bytes() || !source.is_char_boundary(offset) {
            return Err(EditError::InvalidRange);
        }
        self.cursor = offset;
        self.preferred_column = None;
        if !extend {
            self.anchor = offset;
        }
        Ok(())
    }

    pub fn insert(&mut self, source: &mut TextBuffer, text: &str) -> Result<bool, EditError> {
        let range = self.selection();
        let changed = self.history.apply(source, range.clone(), text)?;
        if changed {
            self.cursor = range.start + text.len();
            self.anchor = self.cursor;
        }
        Ok(changed)
    }

    /// Rodea la selección con sintaxis Markdown mediante un único cambio
    /// reversible. Si no hay selección, inserta un texto guía y lo deja
    /// seleccionado para que la siguiente escritura lo reemplace.
    pub fn surround(
        &mut self,
        source: &mut TextBuffer,
        prefix: &str,
        suffix: &str,
        placeholder: &str,
    ) -> Result<bool, EditError> {
        let range = self.selection();
        let selected = source.slice_bytes(range.clone())?;
        let content = if selected.is_empty() {
            placeholder
        } else {
            &selected
        };
        let replacement = format!("{prefix}{content}{suffix}");
        let changed = self.history.apply(source, range.clone(), &replacement)?;
        if changed {
            self.anchor = range.start + prefix.len();
            self.cursor = self.anchor + content.len();
            self.preferred_column = None;
        }
        Ok(changed)
    }

    /// Inserta un enlace Markdown sin consultar el portapapeles ni resolver el
    /// destino. Con texto seleccionado, deja la URL guía seleccionada; sin
    /// selección, deja seleccionado el rótulo para escribirlo primero.
    pub fn insert_link(&mut self, source: &mut TextBuffer) -> Result<bool, EditError> {
        const LABEL: &str = "texto";
        const DESTINATION: &str = "https://";
        let range = self.selection();
        let selected = source.slice_bytes(range.clone())?;
        let had_selection = !selected.is_empty();
        let label = if had_selection { &selected } else { LABEL };
        let replacement = format!("[{label}]({DESTINATION})");
        let changed = self.history.apply(source, range.clone(), &replacement)?;
        if changed {
            if had_selection {
                self.anchor = range.start + 1 + label.len() + 2;
                self.cursor = self.anchor + DESTINATION.len();
            } else {
                self.anchor = range.start + 1;
                self.cursor = self.anchor + LABEL.len();
            }
            self.preferred_column = None;
        }
        Ok(changed)
    }

    pub fn backspace(&mut self, source: &mut TextBuffer) -> Result<bool, EditError> {
        let selection = self.selection();
        let range = if selection.is_empty() {
            let previous = source.previous_boundary(self.cursor);
            previous..self.cursor
        } else {
            selection
        };
        let changed = self.history.apply(source, range.clone(), "")?;
        if changed {
            self.cursor = range.start;
            self.anchor = range.start;
        }
        Ok(changed)
    }

    pub fn delete(&mut self, source: &mut TextBuffer) -> Result<bool, EditError> {
        let selection = self.selection();
        let range = if selection.is_empty() {
            let next = source.next_boundary(self.cursor);
            self.cursor..next
        } else {
            selection
        };
        let changed = self.history.apply(source, range, "")?;
        self.anchor = self.cursor;
        Ok(changed)
    }

    pub fn move_left(&mut self, source: &TextBuffer, extend: bool) -> Result<(), EditError> {
        let target = source.previous_boundary(self.cursor);
        self.set_cursor(source, target, extend)
    }

    pub fn move_right(&mut self, source: &TextBuffer, extend: bool) -> Result<(), EditError> {
        let target = source.next_boundary(self.cursor);
        self.set_cursor(source, target, extend)
    }

    pub fn move_line(
        &mut self,
        source: &TextBuffer,
        down: bool,
        extend: bool,
    ) -> Result<(), EditError> {
        let line = source.rope.byte_to_line(self.cursor);
        let line_start = source.line_start(self.cursor);
        let column = self.preferred_column.unwrap_or(self.cursor - line_start);
        let target_line = if down {
            if line + 1 >= source.rope.len_lines() {
                return Ok(());
            }
            line + 1
        } else if line == 0 {
            return Ok(());
        } else {
            line - 1
        };
        let target_start = source.rope.line_to_byte(target_line);
        let target_end = source.line_end(target_line);
        let mut target = (target_start + column).min(target_end);
        while target > target_start && !source.is_char_boundary(target) {
            target -= 1;
        }
        self.set_cursor(source, target, extend)?;
        self.preferred_column = Some(column);
        Ok(())
    }

    pub fn move_line_boundary(
        &mut self,
        source: &TextBuffer,
        end: bool,
        extend: bool,
    ) -> Result<(), EditError> {
        let target = if end {
            source.line_end(source.rope.byte_to_line(self.cursor))
        } else {
            source.line_start(self.cursor)
        };
        self.set_cursor(source, target, extend)
    }

    pub fn undo(&mut self, source: &mut TextBuffer) -> Result<bool, EditError> {
        let changed = self.history.undo(source)?;
        if changed {
            self.cursor = self.cursor.min(source.len_bytes());
            self.anchor = self.cursor;
        }
        Ok(changed)
    }

    pub fn redo(&mut self, source: &mut TextBuffer) -> Result<bool, EditError> {
        let changed = self.history.redo(source)?;
        if changed {
            self.cursor = self.cursor.min(source.len_bytes());
            self.anchor = self.cursor;
        }
        Ok(changed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn buffer(source: &str) -> TextBuffer {
        TextBuffer::from_text(source)
    }

    #[test]
    fn edita_utf8_y_revierte_sin_tocar_bytes_vecinos() {
        let mut source = buffer("inicio áé final");
        let mut history = EditHistory::new();
        let start = "inicio ".len();
        let end = start + "áé".len();

        assert!(history.apply(&mut source, start..end, "🔒").unwrap());
        assert_eq!(source.to_string(), "inicio 🔒 final");
        assert!(history.is_dirty());
        assert!(history.undo(&mut source).unwrap());
        assert_eq!(source.to_string(), "inicio áé final");
        assert!(!history.is_dirty());
        assert!(history.redo(&mut source).unwrap());
        assert_eq!(source.to_string(), "inicio 🔒 final");
    }

    #[test]
    fn rechaza_rangos_que_partirian_utf8() {
        let mut source = buffer("á");
        let mut history = EditHistory::new();
        assert_eq!(
            history.apply(&mut source, 1..2, "x"),
            Err(EditError::InvalidRange)
        );
        assert_eq!(source.to_string(), "á");
    }

    #[test]
    fn una_edicion_nueva_descarta_el_redo_anterior() {
        let mut source = buffer("abc");
        let mut history = EditHistory::new();
        history.apply(&mut source, 1..2, "B").unwrap();
        history.undo(&mut source).unwrap();
        history.apply(&mut source, 2..3, "C").unwrap();

        assert_eq!(source.to_string(), "abC");
        assert!(!history.can_redo());
    }

    #[test]
    fn ediciones_repetidas_con_unicode_conservan_el_historial_y_los_limites() {
        let mut source = buffer("inicio áéí\nfinal");
        let mut editor = SourceEditor::new();
        let insertion = "inicio ".len();
        editor.set_cursor(&source, insertion, false).unwrap();

        for _ in 0..64 {
            assert!(editor.insert(&mut source, "🔐").unwrap());
        }
        assert!(source.is_char_boundary(editor.cursor()));
        assert_eq!(source.to_string().matches('🔐').count(), 64);

        for _ in 0..64 {
            assert!(editor.undo(&mut source).unwrap());
        }
        assert_eq!(source.to_string(), "inicio áéí\nfinal");
        assert!(!editor.is_dirty());

        for _ in 0..64 {
            assert!(editor.redo(&mut source).unwrap());
        }
        assert_eq!(source.to_string().matches('🔐').count(), 64);
    }

    #[test]
    fn rodear_una_seleccion_unicode_es_un_solo_cambio_reversible() {
        let mut source = buffer("idea segura 🔐");
        let mut editor = SourceEditor::new();
        editor.set_cursor(&source, 0, false).unwrap();
        editor
            .set_cursor(&source, "idea segura".len(), true)
            .unwrap();

        assert!(editor.surround(&mut source, "**", "**", "texto").unwrap());
        assert_eq!(source.to_string(), "**idea segura** 🔐");
        assert_eq!(editor.selection(), 2.."**idea segura".len());
        assert!(editor.undo(&mut source).unwrap());
        assert_eq!(source.to_string(), "idea segura 🔐");
    }

    #[test]
    fn rodear_un_cursor_deja_el_texto_guia_seleccionado() {
        let mut source = buffer("antes después");
        let mut editor = SourceEditor::new();
        editor.set_cursor(&source, "antes ".len(), false).unwrap();

        assert!(editor.surround(&mut source, "_", "_", "texto").unwrap());
        assert_eq!(source.to_string(), "antes _texto_después");
        assert_eq!(source.slice_bytes(editor.selection()).unwrap(), "texto");
    }

    #[test]
    fn insertar_enlace_con_rotulo_unicode_deja_el_destino_para_reemplazar() {
        let mut source = buffer("guía ágil");
        let mut editor = SourceEditor::new();
        editor.set_cursor(&source, 0, false).unwrap();
        editor
            .set_cursor(&source, source.len_bytes(), true)
            .unwrap();

        assert!(editor.insert_link(&mut source).unwrap());
        assert_eq!(source.to_string(), "[guía ágil](https://)");
        assert_eq!(source.slice_bytes(editor.selection()).unwrap(), "https://");
        assert!(editor.undo(&mut source).unwrap());
        assert_eq!(source.to_string(), "guía ágil");
    }

    #[test]
    fn marcar_guardado_distingue_estado_sucio_de_historial() {
        let mut source = buffer("nota");
        let mut history = EditHistory::new();
        history.apply(&mut source, 4..4, " nueva").unwrap();
        history.mark_saved();
        assert!(!history.is_dirty());
        assert!(history.can_undo());
        history.undo(&mut source).unwrap();
        assert!(history.is_dirty());
    }

    #[test]
    fn el_historial_acotado_descarta_undo_antiguo_no_el_documento() {
        let mut source = TextBuffer::new();
        let mut history = EditHistory::new();
        let chunk = "x".repeat(MAX_HISTORY_BYTES / 2 + 1);
        history.apply(&mut source, 0..0, &chunk).unwrap();
        let end = source.len_bytes();
        history.apply(&mut source, end..end, &chunk).unwrap();

        assert_eq!(source.len_bytes(), chunk.len() * 2);
        assert!(history.can_undo());
        history.undo(&mut source).unwrap();
        assert_eq!(source.to_string(), chunk);
        assert!(!history.can_undo());
    }

    #[test]
    fn rehacer_no_duplica_el_presupuesto_del_mismo_cambio() {
        let mut source = TextBuffer::new();
        let mut history = EditHistory::new();
        let chunk = "x".repeat(MAX_HISTORY_BYTES / 3 + 1);
        history.apply(&mut source, 0..0, &chunk).unwrap();
        let end = source.len_bytes();
        history.apply(&mut source, end..end, &chunk).unwrap();

        history.undo(&mut source).unwrap();
        history.redo(&mut source).unwrap();
        history.undo(&mut source).unwrap();
        history.undo(&mut source).unwrap();
        assert!(source.is_empty());
    }

    #[test]
    fn cursor_y_seleccion_editan_fuente_sin_partir_unicode() {
        let mut source = buffer("ábc");
        let mut editor = SourceEditor::new();
        editor.set_cursor(&source, "á".len(), false).unwrap();
        editor
            .set_cursor(&source, source.len_bytes(), true)
            .unwrap();
        assert_eq!(editor.selection(), "á".len()..source.len_bytes());
        editor.insert(&mut source, "🔒").unwrap();
        assert_eq!(source.to_string(), "á🔒");
        editor.backspace(&mut source).unwrap();
        assert_eq!(source.to_string(), "á");
        editor.undo(&mut source).unwrap();
        assert_eq!(source.to_string(), "á🔒");
    }

    #[test]
    fn insertar_crlf_no_normaliza_las_lineas_existentes() {
        let mut source = buffer("uno\r\ndos");
        let mut editor = SourceEditor::new();
        editor.set_cursor(&source, "uno".len(), false).unwrap();
        editor.insert(&mut source, "\r\n").unwrap();
        assert_eq!(source.to_string(), "uno\r\n\r\ndos");
    }

    #[test]
    fn el_buffer_grande_edita_el_centro_sin_corromper_vecinos() {
        let left = "área segura\r\n".repeat(16_384);
        let right = "日本語 y 🔐\n".repeat(16_384);
        let insertion = left.len();
        let original_len = insertion + right.len();
        let mut source = buffer(&(left.clone() + &right));
        let mut editor = SourceEditor::new();

        editor.set_cursor(&source, insertion, false).unwrap();
        assert!(editor.insert(&mut source, "**centro**\r\n").unwrap());
        assert_eq!(source.len_bytes(), original_len + "**centro**\r\n".len());
        assert_eq!(
            source
                .slice_bytes(insertion.saturating_sub(14)..insertion + "**centro**\r\n".len())
                .unwrap(),
            "área segura\r\n**centro**\r\n"
        );
        assert!(editor.undo(&mut source).unwrap());
        assert_eq!(source.len_bytes(), original_len);
        assert_eq!(source.slice_bytes(0..left.len()).unwrap(), left);
        assert_eq!(
            source.slice_bytes(left.len()..source.len_bytes()).unwrap(),
            right
        );
    }

    #[test]
    fn iterar_lineas_conserva_crlf_y_una_linea_final_vacia() {
        let source = buffer("uno\r\ndos\n");
        let lines: Vec<String> = source.lines().map(|line| line.to_string()).collect();
        assert_eq!(lines, ["uno\r\n", "dos\n", ""]);
        assert_eq!(source.to_string(), "uno\r\ndos\n");
    }

    #[test]
    fn navegacion_vertical_conserva_unicode_y_limita_lineas_cortas() {
        let source = buffer("áéí\n文\n🔒🔒🔒");
        let mut editor = SourceEditor::new();
        editor.set_cursor(&source, "áé".len(), false).unwrap();
        editor.move_line(&source, true, false).unwrap();
        assert_eq!(editor.cursor(), "áéí\n文".len());
        editor.move_line(&source, true, false).unwrap();
        assert_eq!(editor.cursor(), "áéí\n文\n🔒".len());
    }

    #[test]
    fn inicio_y_fin_respetan_crlf_y_seleccion() {
        let source = buffer("uno\r\ndos\r\n");
        let mut editor = SourceEditor::new();
        editor.set_cursor(&source, "uno\r\nd".len(), false).unwrap();
        editor.move_line_boundary(&source, false, false).unwrap();
        assert_eq!(editor.cursor(), "uno\r\n".len());
        editor.move_line_boundary(&source, true, true).unwrap();
        assert_eq!(editor.selection(), "uno\r\n".len().."uno\r\ndos\r".len());
    }

    #[test]
    fn seleccionar_todo_abarca_la_fuente_utf8_completa() {
        let source = buffer("á\n🔒");
        let mut editor = SourceEditor::new();
        editor.select_all(&source);
        assert_eq!(editor.selection(), 0..source.len_bytes());
    }

    #[test]
    fn fijar_cursor_sin_extender_cancela_la_seleccion() {
        let source = buffer("ábc");
        let mut editor = SourceEditor::new();
        editor.select_all(&source);
        editor.set_cursor(&source, "á".len(), false).unwrap();
        assert_eq!(editor.selection(), "á".len().."á".len());
    }
}
