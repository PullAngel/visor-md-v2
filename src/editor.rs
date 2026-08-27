//! Modelo de edición de fuente, independiente de parser, layout y ventana.
//!
//! Markdown se modifica como texto UTF-8. No se vuelve a serializar el AST:
//! así una construcción que el lector aún no conoce conserva sus bytes cuando
//! una edición local ocurre a su alrededor.

use std::collections::VecDeque;
use std::ops::Range;

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

    pub fn mark_saved(&mut self) {
        self.saved_revision = self.current_revision;
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
        source: &mut String,
        range: Range<usize>,
        inserted: &str,
    ) -> Result<bool, EditError> {
        if range.start > range.end
            || range.end > source.len()
            || !source.is_char_boundary(range.start)
            || !source.is_char_boundary(range.end)
        {
            return Err(EditError::InvalidRange);
        }

        let removed = source[range.clone()].to_owned();
        if removed == inserted {
            return Ok(false);
        }
        self.clear_redo();
        let before_revision = self.current_revision;
        let after_revision = self.next_revision;
        self.next_revision = self.next_revision.saturating_add(1);
        source.replace_range(range.clone(), inserted);
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

    pub fn undo(&mut self, source: &mut String) -> Result<bool, EditError> {
        let Some(change) = self.undo.pop_back() else {
            return Ok(false);
        };
        if self.current_revision != change.after_revision {
            self.undo.push_back(change);
            return Err(EditError::InconsistentHistory);
        }
        let end = change.start.saturating_add(change.inserted.len());
        if end > source.len()
            || !source.is_char_boundary(change.start)
            || !source.is_char_boundary(end)
            || source.get(change.start..end) != Some(change.inserted.as_str())
        {
            self.undo.push_back(change);
            return Err(EditError::InconsistentHistory);
        }
        source.replace_range(change.start..end, &change.removed);
        self.current_revision = change.before_revision;
        self.redo.push(change);
        Ok(true)
    }

    pub fn redo(&mut self, source: &mut String) -> Result<bool, EditError> {
        let Some(change) = self.redo.pop() else {
            return Ok(false);
        };
        if self.current_revision != change.before_revision {
            self.redo.push(change);
            return Err(EditError::InconsistentHistory);
        }
        let end = change.start.saturating_add(change.removed.len());
        if end > source.len()
            || !source.is_char_boundary(change.start)
            || !source.is_char_boundary(end)
            || source.get(change.start..end) != Some(change.removed.as_str())
        {
            self.redo.push(change);
            return Err(EditError::InconsistentHistory);
        }
        source.replace_range(change.start..end, &change.inserted);
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

    pub fn select_all(&mut self, source: &str) {
        self.anchor = 0;
        self.cursor = source.len();
        self.preferred_column = None;
    }

    pub fn set_cursor(
        &mut self,
        source: &str,
        offset: usize,
        extend: bool,
    ) -> Result<(), EditError> {
        if offset > source.len() || !source.is_char_boundary(offset) {
            return Err(EditError::InvalidRange);
        }
        self.cursor = offset;
        self.preferred_column = None;
        if !extend {
            self.anchor = offset;
        }
        Ok(())
    }

    pub fn insert(&mut self, source: &mut String, text: &str) -> Result<bool, EditError> {
        let range = self.selection();
        let changed = self.history.apply(source, range.clone(), text)?;
        if changed {
            self.cursor = range.start + text.len();
            self.anchor = self.cursor;
        }
        Ok(changed)
    }

    pub fn backspace(&mut self, source: &mut String) -> Result<bool, EditError> {
        let selection = self.selection();
        let range = if selection.is_empty() {
            let previous = source[..self.cursor]
                .char_indices()
                .next_back()
                .map_or(0, |(offset, _)| offset);
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

    pub fn delete(&mut self, source: &mut String) -> Result<bool, EditError> {
        let selection = self.selection();
        let range = if selection.is_empty() {
            let next = source[self.cursor..]
                .char_indices()
                .nth(1)
                .map_or(source.len(), |(offset, _)| self.cursor + offset);
            self.cursor..next
        } else {
            selection
        };
        let changed = self.history.apply(source, range, "")?;
        self.anchor = self.cursor;
        Ok(changed)
    }

    pub fn move_left(&mut self, source: &str, extend: bool) -> Result<(), EditError> {
        let target = source[..self.cursor]
            .char_indices()
            .next_back()
            .map_or(0, |(offset, _)| offset);
        self.set_cursor(source, target, extend)
    }

    pub fn move_right(&mut self, source: &str, extend: bool) -> Result<(), EditError> {
        let target = source[self.cursor..]
            .char_indices()
            .nth(1)
            .map_or(source.len(), |(offset, _)| self.cursor + offset);
        self.set_cursor(source, target, extend)
    }

    pub fn move_line(&mut self, source: &str, down: bool, extend: bool) -> Result<(), EditError> {
        let line_start = source[..self.cursor]
            .rfind('\n')
            .map_or(0, |offset| offset + 1);
        let column = self.preferred_column.unwrap_or(self.cursor - line_start);
        let target_start = if down {
            let Some(next_break) = source[self.cursor..].find('\n') else {
                return Ok(());
            };
            self.cursor + next_break + 1
        } else {
            if line_start == 0 {
                return Ok(());
            }
            source[..line_start.saturating_sub(1)]
                .rfind('\n')
                .map_or(0, |offset| offset + 1)
        };
        let target_end = source[target_start..]
            .find('\n')
            .map_or(source.len(), |offset| target_start + offset);
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
        source: &str,
        end: bool,
        extend: bool,
    ) -> Result<(), EditError> {
        let target = if end {
            source[self.cursor..]
                .find('\n')
                .map_or(source.len(), |offset| self.cursor + offset)
        } else {
            source[..self.cursor]
                .rfind('\n')
                .map_or(0, |offset| offset + 1)
        };
        self.set_cursor(source, target, extend)
    }

    pub fn undo(&mut self, source: &mut String) -> Result<bool, EditError> {
        let changed = self.history.undo(source)?;
        if changed {
            self.cursor = self.cursor.min(source.len());
            self.anchor = self.cursor;
        }
        Ok(changed)
    }

    pub fn redo(&mut self, source: &mut String) -> Result<bool, EditError> {
        let changed = self.history.redo(source)?;
        if changed {
            self.cursor = self.cursor.min(source.len());
            self.anchor = self.cursor;
        }
        Ok(changed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edita_utf8_y_revierte_sin_tocar_bytes_vecinos() {
        let mut source = "inicio áé final".to_owned();
        let mut history = EditHistory::new();
        let start = source.find("áé").unwrap();
        let end = start + "áé".len();

        assert!(history.apply(&mut source, start..end, "🔒").unwrap());
        assert_eq!(source, "inicio 🔒 final");
        assert!(history.is_dirty());
        assert!(history.undo(&mut source).unwrap());
        assert_eq!(source, "inicio áé final");
        assert!(!history.is_dirty());
        assert!(history.redo(&mut source).unwrap());
        assert_eq!(source, "inicio 🔒 final");
    }

    #[test]
    fn rechaza_rangos_que_partirian_utf8() {
        let mut source = "á".to_owned();
        let mut history = EditHistory::new();
        assert_eq!(
            history.apply(&mut source, 1..2, "x"),
            Err(EditError::InvalidRange)
        );
        assert_eq!(source, "á");
    }

    #[test]
    fn una_edicion_nueva_descarta_el_redo_anterior() {
        let mut source = "abc".to_owned();
        let mut history = EditHistory::new();
        history.apply(&mut source, 1..2, "B").unwrap();
        history.undo(&mut source).unwrap();
        history.apply(&mut source, 2..3, "C").unwrap();

        assert_eq!(source, "abC");
        assert!(!history.can_redo());
    }

    #[test]
    fn marcar_guardado_distingue_estado_sucio_de_historial() {
        let mut source = "nota".to_owned();
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
        let mut source = String::new();
        let mut history = EditHistory::new();
        let chunk = "x".repeat(MAX_HISTORY_BYTES / 2 + 1);
        history.apply(&mut source, 0..0, &chunk).unwrap();
        let end = source.len();
        history.apply(&mut source, end..end, &chunk).unwrap();

        assert_eq!(source.len(), chunk.len() * 2);
        assert!(history.can_undo());
        history.undo(&mut source).unwrap();
        assert_eq!(source, chunk);
        assert!(!history.can_undo());
    }

    #[test]
    fn rehacer_no_duplica_el_presupuesto_del_mismo_cambio() {
        let mut source = String::new();
        let mut history = EditHistory::new();
        let chunk = "x".repeat(MAX_HISTORY_BYTES / 3 + 1);
        history.apply(&mut source, 0..0, &chunk).unwrap();
        let end = source.len();
        history.apply(&mut source, end..end, &chunk).unwrap();

        history.undo(&mut source).unwrap();
        history.redo(&mut source).unwrap();
        history.undo(&mut source).unwrap();
        history.undo(&mut source).unwrap();
        assert!(source.is_empty());
    }

    #[test]
    fn cursor_y_seleccion_editan_fuente_sin_partir_unicode() {
        let mut source = "ábc".to_owned();
        let mut editor = SourceEditor::new();
        editor.set_cursor(&source, "á".len(), false).unwrap();
        editor.set_cursor(&source, source.len(), true).unwrap();
        assert_eq!(editor.selection(), "á".len()..source.len());
        editor.insert(&mut source, "🔒").unwrap();
        assert_eq!(source, "á🔒");
        editor.backspace(&mut source).unwrap();
        assert_eq!(source, "á");
        editor.undo(&mut source).unwrap();
        assert_eq!(source, "á🔒");
    }

    #[test]
    fn insertar_crlf_no_normaliza_las_lineas_existentes() {
        let mut source = "uno\r\ndos".to_owned();
        let mut editor = SourceEditor::new();
        editor.set_cursor(&source, "uno".len(), false).unwrap();
        editor.insert(&mut source, "\r\n").unwrap();
        assert_eq!(source, "uno\r\n\r\ndos");
    }

    #[test]
    fn navegacion_vertical_conserva_unicode_y_limita_lineas_cortas() {
        let source = "áéí\n文\n🔒🔒🔒";
        let mut editor = SourceEditor::new();
        editor.set_cursor(source, "áé".len(), false).unwrap();
        editor.move_line(source, true, false).unwrap();
        assert_eq!(editor.cursor(), "áéí\n文".len());
        editor.move_line(source, true, false).unwrap();
        assert_eq!(editor.cursor(), "áéí\n文\n🔒".len());
    }

    #[test]
    fn inicio_y_fin_respetan_crlf_y_seleccion() {
        let source = "uno\r\ndos\r\n";
        let mut editor = SourceEditor::new();
        editor.set_cursor(source, "uno\r\nd".len(), false).unwrap();
        editor.move_line_boundary(source, false, false).unwrap();
        assert_eq!(editor.cursor(), "uno\r\n".len());
        editor.move_line_boundary(source, true, true).unwrap();
        assert_eq!(editor.selection(), "uno\r\n".len().."uno\r\ndos\r".len());
    }

    #[test]
    fn seleccionar_todo_abarca_la_fuente_utf8_completa() {
        let source = "á\n🔒";
        let mut editor = SourceEditor::new();
        editor.select_all(source);
        assert_eq!(editor.selection(), 0..source.len());
    }

    #[test]
    fn fijar_cursor_sin_extender_cancela_la_seleccion() {
        let source = "ábc";
        let mut editor = SourceEditor::new();
        editor.select_all(source);
        editor.set_cursor(source, "á".len(), false).unwrap();
        assert_eq!(editor.selection(), "á".len().."á".len());
    }
}
