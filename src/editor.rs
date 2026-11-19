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
    /// Una copia barata del texto (`Rope::clone` es O(1)): sirve para guardar lo
    /// último escrito en el registro del gancho de pánico.
    pub fn snapshot(&self) -> Rope {
        self.rope.clone()
    }

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

    /// Un CRLF representa un único salto de línea para quien edita. Aunque
    /// ambos bytes son límites UTF-8 válidos, dejar el cursor entre `\r` y
    /// `\n` permitiría borrar o insertar solo una mitad del salto Windows.
    fn is_crlf_interior(&self, byte: usize) -> bool {
        byte > 0
            && byte < self.len_bytes()
            && self.rope.byte(byte - 1) == b'\r'
            && self.rope.byte(byte) == b'\n'
    }

    /// Límite permitido para una interacción o un parche externo. Es más
    /// estricto que un límite UTF-8: también mantiene CRLF como unidad atómica.
    fn is_edit_boundary(&self, byte: usize) -> bool {
        self.is_char_boundary(byte) && !self.is_crlf_interior(byte)
    }

    /// Los cambios internos de historial pueden recrear temporalmente un
    /// CRLF en el borde de un parche. La interfaz siempre se coloca después
    /// del salto completo, nunca entre sus dos bytes.
    fn normalize_edit_boundary(&self, byte: usize) -> usize {
        let byte = byte.min(self.len_bytes());
        if self.is_crlf_interior(byte) {
            byte + 1
        } else {
            byte
        }
    }

    pub fn slice_bytes(&self, range: Range<usize>) -> Result<String, EditError> {
        if range.start > range.end
            || !self.is_edit_boundary(range.start)
            || !self.is_edit_boundary(range.end)
        {
            return Err(EditError::InvalidRange);
        }
        self.slice_bytes_at_char_boundaries(range)
    }

    /// Ruta interna para revertir un parche de historial exacto. No se expone
    /// a la interacción: solo permite deshacer una edición que convirtió dos
    /// bytes vecinos en un CRLF antes de volver a su estado original.
    fn slice_bytes_at_char_boundaries(&self, range: Range<usize>) -> Result<String, EditError> {
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

    fn replace_range_at_char_boundaries(
        &mut self,
        range: Range<usize>,
        inserted: &str,
    ) -> Result<(), EditError> {
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
        let byte = self.normalize_edit_boundary(byte);
        let character = self.rope.byte_to_char(byte);
        let previous = self.rope.char_to_byte(character.saturating_sub(1));
        if previous > 0
            && self.rope.byte(previous) == b'\n'
            && self.rope.byte(previous - 1) == b'\r'
        {
            previous - 1
        } else {
            previous
        }
    }

    fn next_boundary(&self, byte: usize) -> usize {
        let byte = self.normalize_edit_boundary(byte);
        if byte < self.len_bytes()
            && self.rope.byte(byte) == b'\r'
            && byte + 1 < self.len_bytes()
            && self.rope.byte(byte + 1) == b'\n'
        {
            return byte + 2;
        }
        let character = self.rope.byte_to_char(byte);
        self.rope
            .char_to_byte((character + 1).min(self.rope.len_chars()))
    }

    fn char_at_byte(&self, byte: usize) -> Option<char> {
        (byte < self.len_bytes())
            .then(|| self.rope.get_char(self.rope.byte_to_char(byte)))
            .flatten()
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
        // CRLF representa un único salto de línea para la interacción. El
        // buffer conserva ambos bytes al guardar, pero el cursor nunca queda
        // artificialmente entre `\r` y `\n`.
        if end > start && self.rope.byte(end - 1) == b'\r' {
            end -= 1;
        }
        end
    }

    /// Columna de edición en caracteres Unicode, no en bytes UTF-8. Es una
    /// unidad de navegación conservadora sin añadir segmentación de grafemas
    /// al núcleo; evita que acentos o ideogramas desplacen una flecha vertical.
    fn line_column(&self, byte: usize) -> usize {
        let byte = byte.min(self.len_bytes());
        let line = self.rope.byte_to_line(byte);
        self.rope.byte_to_char(byte) - self.rope.line_to_char(line)
    }

    fn byte_at_line_column(&self, line: usize, column: usize) -> usize {
        let line = line.min(self.rope.len_lines().saturating_sub(1));
        let start = self.rope.line_to_char(line);
        let end = self.rope.byte_to_char(self.line_end(line));
        self.rope.char_to_byte((start + column).min(end))
    }

    pub fn lines(&self) -> ropey::iter::Lines<'_> {
        self.rope.lines()
    }

    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    pub fn line_at_byte(&self, byte: usize) -> usize {
        self.rope.byte_to_line(byte.min(self.len_bytes()))
    }

    pub fn line_start_byte(&self, line: usize) -> usize {
        self.rope
            .line_to_byte(line.min(self.rope.len_lines().saturating_sub(1)))
    }

    pub fn line_text(&self, line: usize) -> String {
        self.rope
            .line(line.min(self.rope.len_lines().saturating_sub(1)))
            .to_string()
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

/// Descripción compacta del último parche aplicado al buffer. Permite que una
/// vista derivada actualice solo su vecindario, sin convertirla en autoridad
/// sobre la fuente ni guardar snapshots completos.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceChange {
    pub start: usize,
    pub removed_bytes: usize,
    pub inserted_bytes: usize,
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
    last_change: Option<SourceChange>,
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
            last_change: None,
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
            || !source.is_edit_boundary(range.start)
            || !source.is_edit_boundary(range.end)
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
        let removed_bytes = removed.len();
        self.next_revision = self.next_revision.saturating_add(1);
        source.replace_range_at_char_boundaries(range.clone(), inserted)?;
        let change = Change {
            start: range.start,
            removed,
            inserted: inserted.to_owned(),
            before_revision,
            after_revision,
        };
        self.current_revision = after_revision;
        self.last_change = Some(SourceChange {
            start: range.start,
            removed_bytes,
            inserted_bytes: inserted.len(),
        });
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
            || source
                .slice_bytes_at_char_boundaries(change.start..end)?
                .as_str()
                != change.inserted
        {
            self.undo.push_back(change);
            return Err(EditError::InconsistentHistory);
        }
        source.replace_range_at_char_boundaries(change.start..end, &change.removed)?;
        self.current_revision = change.before_revision;
        self.last_change = Some(SourceChange {
            start: change.start,
            removed_bytes: change.inserted.len(),
            inserted_bytes: change.removed.len(),
        });
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
            || source
                .slice_bytes_at_char_boundaries(change.start..end)?
                .as_str()
                != change.removed
        {
            self.redo.push(change);
            return Err(EditError::InconsistentHistory);
        }
        source.replace_range_at_char_boundaries(change.start..end, &change.inserted)?;
        self.current_revision = change.after_revision;
        self.last_change = Some(SourceChange {
            start: change.start,
            removed_bytes: change.removed.len(),
            inserted_bytes: change.inserted.len(),
        });
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

    fn last_change(&self) -> Option<SourceChange> {
        self.last_change
    }
}

/// Prefijos de bloque que el kit de escritura aplica sobre líneas.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinePrefix {
    Bullet,
    Ordered,
    Task,
    Quote,
}

impl LinePrefix {
    fn marker(self, number: usize) -> String {
        match self {
            Self::Bullet => "- ".to_string(),
            Self::Ordered => format!("{number}. "),
            Self::Task => "- [ ] ".to_string(),
            Self::Quote => "> ".to_string(),
        }
    }
}

struct ListMarker {
    /// Bytes del inicio de línea que ocupa la sangría más el marcador.
    len: usize,
    /// Marcador que abre el elemento siguiente, con la misma sangría.
    next: String,
}

/// Reconoce el marcador de lista o cita al inicio de una línea, siempre que el
/// cursor esté después de él. No interpreta contenido: solo la sintaxis del
/// marcador, para que Enter no invente estructura en texto común.
fn list_marker(before_cursor: &str) -> Option<ListMarker> {
    let indent_len = before_cursor.len() - before_cursor.trim_start_matches([' ', '\t']).len();
    let indent = &before_cursor[..indent_len];
    let rest = &before_cursor[indent_len..];
    let (marker_len, next) = if rest.starts_with("> ") {
        (2, "> ".to_string())
    } else if let Some(bullet) = ["- ", "* ", "+ "]
        .iter()
        .find(|bullet| rest.starts_with(**bullet))
    {
        let after = &rest[2..];
        if after.starts_with("[ ] ") || after.starts_with("[x] ") || after.starts_with("[X] ") {
            (6, format!("{bullet}[ ] "))
        } else {
            (2, (*bullet).to_string())
        }
    } else {
        let digits = rest.bytes().take_while(u8::is_ascii_digit).count();
        if digits == 0 || digits > 9 {
            return None;
        }
        let delimiter = *rest.as_bytes().get(digits)?;
        if !matches!(delimiter, b'.' | b')') || rest.as_bytes().get(digits + 1) != Some(&b' ') {
            return None;
        }
        let number: u64 = rest[..digits].parse().ok()?;
        (digits + 2, format!("{}{} ", number + 1, delimiter as char))
    };
    (rest.len() >= marker_len).then(|| ListMarker {
        len: indent_len + marker_len,
        next: format!("{indent}{next}"),
    })
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

    /// Devuelve la selección exacta de la fuente. La copia desde edición no
    /// debe depender de una vista Markdown que puede estar esperando su
    /// actualización asíncrona.
    pub fn selected_text(&self, source: &TextBuffer) -> Result<Option<String>, EditError> {
        let selection = self.selection();
        (!selection.is_empty())
            .then(|| source.slice_bytes(selection))
            .transpose()
    }

    pub fn is_dirty(&self) -> bool {
        self.history.is_dirty()
    }

    pub fn revision(&self) -> u64 {
        self.history.current_revision()
    }

    pub fn last_change(&self) -> Option<SourceChange> {
        self.history.last_change()
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
        if offset > source.len_bytes() || !source.is_edit_boundary(offset) {
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
            self.cursor = source.normalize_edit_boundary(range.start + text.len());
            self.anchor = self.cursor;
        }
        Ok(changed)
    }

    /// Inserta un símbolo o emoji. Si el cursor está después del `|` final de
    /// una fila de tabla, lo coloca dentro de la última celda: fuera de ella la
    /// fila dejaría de ser una fila y el símbolo desaparecería de la lectura.
    pub fn insert_symbol(
        &mut self,
        source: &mut TextBuffer,
        symbol: &str,
    ) -> Result<bool, EditError> {
        if self.selection().is_empty()
            && let Some(position) = table_row_content_end(source, self.cursor)?
        {
            self.cursor = position;
            self.anchor = position;
            return self.insert(source, &format!(" {symbol}"));
        }
        self.insert(source, symbol)
    }

    /// Inserta un bloque Markdown (tabla, regla, recuadro) en su propia línea,
    /// con una línea en blanco antes y después cuando hace falta. Sin esa
    /// separación el bloque se pega al párrafo vecino y deja de reconocerse.
    /// Nunca reemplaza texto seleccionado ni parte una línea: va debajo de la
    /// línea del cursor o, si el cursor está al inicio de una línea con texto,
    /// encima de ella. `select` es el rango de la plantilla que queda
    /// seleccionado; sin él, el cursor queda al final del bloque.
    pub fn insert_block(
        &mut self,
        source: &mut TextBuffer,
        template: &str,
        eol: &str,
        select: Option<Range<usize>>,
    ) -> Result<bool, EditError> {
        let at = block_anchor(source, self.selection().end)?;
        self.place_block(source, at..at, template, eol, select)
    }

    fn place_block(
        &mut self,
        source: &mut TextBuffer,
        range: Range<usize>,
        template: &str,
        eol: &str,
        select: Option<Range<usize>>,
    ) -> Result<bool, EditError> {
        let (lead, trail) = block_padding(source, &range, eol)?;
        let text = format!("{lead}{template}{trail}");
        let changed = self.history.apply(source, range.clone(), &text)?;
        if changed {
            let base = range.start + lead.len();
            let (anchor, cursor) = match select {
                Some(inner) => (base + inner.start, base + inner.end),
                None => (base + template.len(), base + template.len()),
            };
            self.anchor = source.normalize_edit_boundary(anchor);
            self.cursor = source.normalize_edit_boundary(cursor);
            self.preferred_column = None;
        }
        Ok(changed)
    }

    /// Bloque de código con cercas. Con selección, la envuelve; sin ella,
    /// deja un texto guía seleccionado.
    pub fn insert_code_block(
        &mut self,
        source: &mut TextBuffer,
        eol: &str,
    ) -> Result<bool, EditError> {
        let selection = self.selection();
        let selected = source.slice_bytes(selection.clone())?;
        let content = if selected.is_empty() {
            "código"
        } else {
            selected.as_str()
        };
        let template = format!("```{eol}{content}{eol}```");
        let start = 3 + eol.len();
        let inner = Some(start..start + content.len());
        if selected.is_empty() {
            self.insert_block(source, &template, eol, inner)
        } else {
            self.place_block(source, selection, &template, eol, inner)
        }
    }

    /// Rodea la selección con sintaxis Markdown mediante un único cambio
    /// reversible. Sin selección, si el cursor está dentro o al final de una
    /// palabra, rodea esa palabra entera; si no, inserta un texto guía y lo
    /// deja seleccionado para que la siguiente escritura lo reemplace.
    pub fn surround(
        &mut self,
        source: &mut TextBuffer,
        prefix: &str,
        suffix: &str,
        placeholder: &str,
    ) -> Result<bool, EditError> {
        let mut range = self.selection();
        if range.is_empty()
            && let Some(word) = word_range_at(source, range.start)?
        {
            range = word;
        }
        let selected = source.slice_bytes(range.clone())?;
        let content = if selected.is_empty() {
            placeholder
        } else {
            &selected
        };
        let replacement = format!("{prefix}{content}{suffix}");
        let changed = self.history.apply(source, range.clone(), &replacement)?;
        if changed {
            self.anchor = source.normalize_edit_boundary(range.start + prefix.len());
            self.cursor = source.normalize_edit_boundary(self.anchor + content.len());
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
                self.anchor = source.normalize_edit_boundary(range.start + 1 + label.len() + 2);
                self.cursor = source.normalize_edit_boundary(self.anchor + DESTINATION.len());
            } else {
                self.anchor = source.normalize_edit_boundary(range.start + 1);
                self.cursor = source.normalize_edit_boundary(self.anchor + LABEL.len());
            }
            self.preferred_column = None;
        }
        Ok(changed)
    }

    /// Inserta un marcador al inicio de la línea actual. La operación no
    /// interpreta ni normaliza las demás líneas y mantiene selección y cursor
    /// sobre el mismo contenido lógico.
    pub fn prefix_current_line(
        &mut self,
        source: &mut TextBuffer,
        prefix: &str,
    ) -> Result<bool, EditError> {
        let start = source.line_start(self.cursor);
        let changed = self.history.apply(source, start..start, prefix)?;
        if changed {
            self.cursor = source.normalize_edit_boundary(self.cursor + prefix.len());
            if self.anchor >= start {
                self.anchor = source.normalize_edit_boundary(self.anchor + prefix.len());
            }
            self.preferred_column = None;
        }
        Ok(changed)
    }

    /// Enter con continuación de listas y citas. Sobre un elemento con texto
    /// abre el siguiente (numerado +1, tarea vacía, misma viñeta o cita); sobre
    /// un elemento vacío termina la lista quitando su marcador. Con selección,
    /// o fuera de una lista, inserta un salto normal.
    pub fn insert_newline(
        &mut self,
        source: &mut TextBuffer,
        eol: &str,
    ) -> Result<bool, EditError> {
        if !self.selection().is_empty() {
            return self.insert(source, eol);
        }
        let start = source.line_start(self.cursor);
        let line = source.line_at_byte(self.cursor);
        let end = source.line_end(line);
        let before = source.slice_bytes(start..self.cursor)?;
        let Some(marker) = list_marker(&before) else {
            return self.insert(source, eol);
        };
        let content_after_cursor = source.slice_bytes(self.cursor..end)?;
        if before.len() == marker.len && content_after_cursor.trim().is_empty() {
            // Elemento vacío: Enter sale de la lista. Si sigue a otro elemento,
            // el marcador se reemplaza por una línea en blanco: sin ella, lo
            // que se escriba después sería continuación del último elemento.
            let follows_item = start > 0 && {
                let previous_start = source.line_start(start - 1);
                let previous = source.slice_bytes(previous_start..start)?;
                list_marker(previous.trim_end_matches(['\r', '\n'])).is_some()
            };
            let replacement = if follows_item { eol } else { "" };
            let changed = self.history.apply(source, start..end, replacement)?;
            if changed {
                self.cursor = start + replacement.len();
                self.anchor = self.cursor;
                self.preferred_column = None;
            }
            return Ok(changed);
        }
        let continuation = format!("{eol}{}", marker.next);
        self.insert(source, &continuation)
    }

    /// Aplica un prefijo de lista o cita respetando la selección:
    ///
    /// - sin selección, prefija la línea actual (o empieza un elemento en una
    ///   línea vacía);
    /// - con una parte de una sola línea, separa esa parte como un elemento
    ///   propio y deja intacto el resto del párrafo;
    /// - con varias líneas, prefija cada línea con texto, numerando en orden.
    pub fn apply_line_prefix(
        &mut self,
        source: &mut TextBuffer,
        kind: LinePrefix,
        eol: &str,
    ) -> Result<bool, EditError> {
        let selection = self.selection();
        if selection.is_empty() {
            let start = source.line_start(self.cursor);
            let prefix = kind.marker(1);
            let changed = self.history.apply(source, start..start, &prefix)?;
            if changed {
                self.cursor = source.normalize_edit_boundary(self.cursor + prefix.len());
                self.anchor = self.cursor;
                self.preferred_column = None;
            }
            return Ok(changed);
        }
        let first_line = source.line_at_byte(selection.start);
        let mut last_line = source.line_at_byte(selection.end);
        if last_line > first_line && source.line_start(selection.end) == selection.end {
            // Una selección que termina al inicio de una línea no la incluye.
            last_line -= 1;
        }
        let first_start = source.line_start(selection.start);
        let last_end = source.line_end(last_line);
        if first_line == last_line {
            let whole_line = source
                .slice_bytes(first_start..selection.start)?
                .trim()
                .is_empty()
                && source
                    .slice_bytes(selection.end..last_end)?
                    .trim()
                    .is_empty();
            if !whole_line {
                let selected = source.slice_bytes(selection.clone())?;
                let selected = selected.trim();
                let leading = if source
                    .slice_bytes(first_start..selection.start)?
                    .trim()
                    .is_empty()
                {
                    String::new()
                } else {
                    eol.to_string()
                };
                let trailing = if source
                    .slice_bytes(selection.end..last_end)?
                    .trim()
                    .is_empty()
                {
                    String::new()
                } else {
                    eol.to_string()
                };
                let replacement = format!("{leading}{}{selected}{trailing}", kind.marker(1));
                let changed = self
                    .history
                    .apply(source, selection.clone(), &replacement)?;
                if changed {
                    let item_start = selection.start + leading.len();
                    self.anchor = item_start;
                    self.cursor = source.normalize_edit_boundary(
                        item_start + kind.marker(1).len() + selected.len(),
                    );
                    self.preferred_column = None;
                }
                return Ok(changed);
            }
        }
        let original = source.slice_bytes(first_start..last_end)?;
        let mut number = 0;
        let mut replacement =
            String::with_capacity(original.len() + 8 * (last_line - first_line + 1));
        for piece in original.split_inclusive('\n') {
            let (body, ending) = match piece.strip_suffix("\r\n") {
                Some(body) => (body, "\r\n"),
                None => match piece.strip_suffix('\n') {
                    Some(body) => (body, "\n"),
                    None => (piece, ""),
                },
            };
            if body.trim().is_empty() {
                replacement.push_str(body);
            } else {
                number += 1;
                let indent = body.len() - body.trim_start().len();
                replacement.push_str(&body[..indent]);
                replacement.push_str(&kind.marker(number));
                replacement.push_str(&body[indent..]);
            }
            replacement.push_str(ending);
        }
        let changed = self
            .history
            .apply(source, first_start..last_end, &replacement)?;
        if changed {
            self.anchor = first_start;
            self.cursor = source.normalize_edit_boundary(first_start + replacement.len());
            self.preferred_column = None;
        }
        Ok(changed)
    }

    /// Establece un encabezado ATX entre H1 y H6 sobre la línea actual. Si la
    /// línea ya empieza con un encabezado válido, reemplaza únicamente ese
    /// marcador; no acumula almohadillas ni reescribe el contenido restante.
    pub fn set_heading_level(
        &mut self,
        source: &mut TextBuffer,
        level: u8,
    ) -> Result<bool, EditError> {
        let level = level.clamp(1, 6) as usize;
        let line = source.line_at_byte(self.cursor);
        let line_start = source.line_start_byte(line);
        let line_text = source.line_text(line);
        let bytes = line_text.as_bytes();
        let indent = bytes
            .iter()
            .take(3)
            .take_while(|byte| **byte == b' ')
            .count();
        let hashes = bytes[indent..]
            .iter()
            .take(6)
            .take_while(|byte| **byte == b'#')
            .count();
        let marker_bytes = if hashes > 0 && bytes.get(indent + hashes) == Some(&b' ') {
            hashes + 1
        } else {
            0
        };
        let range = line_start + indent..line_start + indent + marker_bytes;
        let marker = format!("{} ", "#".repeat(level));
        let changed = self.history.apply(source, range.clone(), &marker)?;
        if changed {
            let rebase = |position: usize| {
                if position <= range.start {
                    position
                } else if position >= range.end {
                    range.start + marker.len() + position - range.end
                } else {
                    range.start + marker.len()
                }
            };
            self.cursor = source.normalize_edit_boundary(rebase(self.cursor));
            self.anchor = source.normalize_edit_boundary(rebase(self.anchor));
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
            self.cursor = source.normalize_edit_boundary(range.start);
            self.anchor = range.start;
            self.anchor = source.normalize_edit_boundary(self.anchor);
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
        self.cursor = source.normalize_edit_boundary(self.cursor);
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
        let column = self
            .preferred_column
            .unwrap_or_else(|| source.line_column(self.cursor));
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
        let target = source.byte_at_line_column(target_line, column);
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

    /// Navegación semántica pequeña y local: letras, números y guion bajo
    /// forman una palabra. No necesita segmentadores externos y conserva la
    /// garantía de no ubicar el cursor dentro de UTF-8 o CRLF.
    pub fn move_word(
        &mut self,
        source: &TextBuffer,
        forward: bool,
        extend: bool,
    ) -> Result<(), EditError> {
        let mut target = self.cursor;
        if forward {
            while source.char_at_byte(target).is_some_and(is_word_character) {
                target = source.next_boundary(target);
            }
            while source
                .char_at_byte(target)
                .is_some_and(|character| !is_word_character(character))
            {
                target = source.next_boundary(target);
            }
        } else {
            while target > 0 {
                let previous = source.previous_boundary(target);
                if source
                    .char_at_byte(previous)
                    .is_some_and(|character| !is_word_character(character))
                {
                    target = previous;
                } else {
                    break;
                }
            }
            while target > 0 {
                let previous = source.previous_boundary(target);
                if source.char_at_byte(previous).is_some_and(is_word_character) {
                    target = previous;
                } else {
                    break;
                }
            }
        }
        self.set_cursor(source, target, extend)
    }

    /// Va al límite del documento sin usar bytes intermedios de una secuencia
    /// UTF-8 ni del CRLF. `Shift` conserva el ancla para la selección habitual
    /// de Ctrl+Inicio y Ctrl+Fin.
    pub fn move_document_boundary(
        &mut self,
        source: &TextBuffer,
        end: bool,
        extend: bool,
    ) -> Result<(), EditError> {
        let target = if end { source.len_bytes() } else { 0 };
        self.set_cursor(source, target, extend)
    }

    pub fn undo(&mut self, source: &mut TextBuffer) -> Result<bool, EditError> {
        let changed = self.history.undo(source)?;
        if changed {
            // El cursor pertenece al cambio que se acaba de revertir, no a la
            // posición posterior que tenía antes de deshacer. Mantener aquella
            // posición podía hacerlo saltar visualmente hacia delante.
            self.cursor = self
                .history
                .last_change()
                .map(|change| change.start + change.inserted_bytes)
                .unwrap_or(0)
                .min(source.len_bytes());
            self.cursor = source.normalize_edit_boundary(self.cursor);
            self.anchor = self.cursor;
        }
        Ok(changed)
    }

    pub fn redo(&mut self, source: &mut TextBuffer) -> Result<bool, EditError> {
        let changed = self.history.redo(source)?;
        if changed {
            self.cursor = self
                .history
                .last_change()
                .map(|change| change.start + change.inserted_bytes)
                .unwrap_or(0)
                .min(source.len_bytes());
            self.cursor = source.normalize_edit_boundary(self.cursor);
            self.anchor = self.cursor;
        }
        Ok(changed)
    }
}

/// Fin del contenido de una fila de tabla (`| a | b |`) cuando el cursor está
/// después de su `|` final: el desplazamiento justo tras el último texto de la
/// última celda. Con el cursor en cualquier otro lugar, `None`.
fn table_row_content_end(source: &TextBuffer, offset: usize) -> Result<Option<usize>, EditError> {
    let line = source.line_at_byte(offset);
    let line_start = source.line_start_byte(line);
    let text = source.slice_bytes(line_start..source.line_end(line))?;
    let row = text.trim_end();
    if row.len() < 2 || !text.trim_start().starts_with('|') || !row.ends_with('|') {
        return Ok(None);
    }
    if offset.saturating_sub(line_start) < row.len() {
        return Ok(None);
    }
    let inner = row[..row.len() - 1].trim_end().len();
    Ok(Some(line_start + inner))
}

/// Palabra (letras, números o guion bajo) que contiene el desplazamiento o
/// termina en él, sin salir de la línea. Con el cursor justo antes de una
/// palabra no se elige ninguna: ahí la persona empieza algo nuevo.
fn word_range_at(source: &TextBuffer, offset: usize) -> Result<Option<Range<usize>>, EditError> {
    let line = source.line_at_byte(offset);
    let line_start = source.line_start_byte(line);
    let text = source.slice_bytes(line_start..source.line_end(line))?;
    let at = offset.saturating_sub(line_start);
    if at > text.len() || !text.is_char_boundary(at) {
        return Ok(None);
    }
    let before: usize = text[..at]
        .chars()
        .rev()
        .take_while(|character| is_word_character(*character))
        .map(char::len_utf8)
        .sum();
    let after: usize = text[at..]
        .chars()
        .take_while(|character| is_word_character(*character))
        .map(char::len_utf8)
        .sum();
    if before == 0 {
        return Ok(None);
    }
    Ok(Some(line_start + at - before..line_start + at + after))
}

/// Dónde colocar un bloque nuevo respecto del cursor: en una línea en blanco,
/// el propio cursor; al inicio de una línea con texto, el inicio de esa línea
/// (el bloque queda encima); en cualquier otro caso, el final de la línea
/// (el bloque queda debajo). Así un bloque nunca parte una línea.
fn block_anchor(source: &TextBuffer, caret: usize) -> Result<usize, EditError> {
    let line = source.line_at_byte(caret);
    let line_start = source.line_start_byte(line);
    let line_end = source.line_end(line);
    let full = source.slice_bytes(line_start..line_end)?;
    if full.trim().is_empty() {
        return Ok(caret);
    }
    let before = source.slice_bytes(line_start..caret.min(line_end))?;
    Ok(if before.trim().is_empty() {
        line_start
    } else {
        line_end
    })
}

/// Saltos que necesita un bloque insertado en `range` para quedar aislado:
/// una línea en blanco antes y otra después, salvo al inicio o al final del
/// documento o si ya hay una.
fn block_padding(
    source: &TextBuffer,
    range: &Range<usize>,
    eol: &str,
) -> Result<(String, String), EditError> {
    let line_start = source.line_start(range.start);
    let before = source.slice_bytes(line_start..range.start)?;
    let lead = if !before.trim().is_empty() {
        format!("{eol}{eol}")
    } else if line_start > 0 {
        let previous_start = source.line_start(line_start - 1);
        let previous = source.slice_bytes(previous_start..line_start)?;
        if previous.trim().is_empty() {
            String::new()
        } else {
            eol.to_string()
        }
    } else {
        String::new()
    };

    let end_line = source.line_at_byte(range.end);
    let after = source.slice_bytes(range.end..source.line_end(end_line))?;
    let trail = if !after.trim().is_empty() {
        format!("{eol}{eol}")
    } else if end_line + 1 < source.line_count() {
        let next_start = source.line_start_byte(end_line + 1);
        let next = source.slice_bytes(next_start..source.line_end(end_line + 1))?;
        if next.trim().is_empty() {
            String::new()
        } else {
            eol.to_string()
        }
    } else {
        String::new()
    };
    Ok((lead, trail))
}

fn is_word_character(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    fn buffer(source: &str) -> TextBuffer {
        TextBuffer::from_text(source)
    }

    fn editor_at(source: &TextBuffer, offset: usize) -> SourceEditor {
        let mut editor = SourceEditor::new();
        editor.set_cursor(source, offset, false).unwrap();
        editor
    }

    #[test]
    fn enter_continua_listas_numeradas_tareas_y_citas() {
        for (text, expected) in [
            ("1. uno", "1. uno\n2. "),
            ("  9) nueve", "  9) nueve\n  10) "),
            ("- viñeta", "- viñeta\n- "),
            ("* [x] hecha", "* [x] hecha\n* [ ] "),
            ("> cita", "> cita\n> "),
            ("texto común", "texto común\n"),
        ] {
            let mut source = buffer(text);
            let mut editor = editor_at(&source, text.len());
            assert!(editor.insert_newline(&mut source, "\n").unwrap());
            assert_eq!(source.to_string(), expected, "{text}");
            assert_eq!(editor.cursor(), expected.len());
        }
    }

    #[test]
    fn enter_en_un_elemento_vacio_termina_la_lista_con_una_linea_en_blanco() {
        for (text, expected) in [
            ("- uno\n- ", "- uno\n\n"),
            ("1. uno\n2. ", "1. uno\n\n"),
            ("- [ ] uno\n- [ ] ", "- [ ] uno\n\n"),
            ("> cita\n> ", "> cita\n\n"),
        ] {
            let mut source = buffer(text);
            let mut editor = editor_at(&source, source.len_bytes());
            assert!(editor.insert_newline(&mut source, "\n").unwrap(), "{text}");
            assert_eq!(source.to_string(), expected, "{text}");
            assert_eq!(editor.cursor(), expected.len(), "{text}");
            assert!(editor.undo(&mut source).unwrap());
            assert_eq!(source.to_string(), text);
        }
    }

    #[test]
    fn enter_en_un_marcador_vacio_aislado_solo_lo_quita() {
        let mut source = buffer("- ");
        let mut editor = editor_at(&source, source.len_bytes());
        assert!(editor.insert_newline(&mut source, "\n").unwrap());
        assert_eq!(source.to_string(), "");

        let mut source = buffer("texto\n- ");
        let mut editor = editor_at(&source, source.len_bytes());
        assert!(editor.insert_newline(&mut source, "\n").unwrap());
        assert_eq!(source.to_string(), "texto\n");
    }

    fn simbolo_en(text: &str, offset: usize) -> String {
        let mut source = buffer(text);
        let mut editor = editor_at(&source, offset);
        assert!(editor.insert_symbol(&mut source, "✓").unwrap());
        source.to_string()
    }

    #[test]
    fn un_simbolo_tras_el_pipe_final_entra_en_la_ultima_celda() {
        let fila = "| texto | valor |";
        assert_eq!(simbolo_en(fila, fila.len()), "| texto | valor ✓ |");
        assert_eq!(
            simbolo_en("| a | b |  ", "| a | b |  ".len()),
            "| a | b ✓ |  "
        );
        assert_eq!(simbolo_en("| a | b |", "| a | b |".len()), "| a | b ✓ |");
        // Sin espacio antes del pipe final.
        assert_eq!(simbolo_en("|a|b|", 5), "|a|b ✓|");
        // CRLF: el salto no cuenta como parte de la fila.
        assert_eq!(simbolo_en("| a |\r\nsigue", 5), "| a ✓ |\r\nsigue");
    }

    #[test]
    fn un_simbolo_fuera_del_final_de_una_fila_se_inserta_donde_esta_el_cursor() {
        // En medio de una celda.
        assert_eq!(simbolo_en("| texto | valor |", 4), "| te✓xto | valor |");
        // Línea que no es una fila.
        assert_eq!(simbolo_en("texto |", 7), "texto |✓");
        // Fila sin pipe final.
        assert_eq!(simbolo_en("| a | b", 7), "| a | b✓");
        // Un solo pipe.
        assert_eq!(simbolo_en("|", 1), "|✓");
    }

    #[test]
    fn un_simbolo_en_una_fila_se_deshace_de_una_vez() {
        let mut source = buffer("| a |");
        let mut editor = editor_at(&source, source.len_bytes());
        assert!(editor.insert_symbol(&mut source, "✓").unwrap());
        assert!(editor.undo(&mut source).unwrap());
        assert_eq!(source.to_string(), "| a |");
    }

    #[test]
    fn salir_de_una_lista_conserva_crlf() {
        let mut source = buffer("- uno\r\n- ");
        let mut editor = editor_at(&source, source.len_bytes());
        assert!(editor.insert_newline(&mut source, "\r\n").unwrap());
        assert_eq!(source.to_string(), "- uno\r\n\r\n");
        assert_eq!(editor.cursor(), source.len_bytes());
    }

    fn insertar_bloque(text: &str, offset: usize, eol: &str) -> (String, usize) {
        let mut source = buffer(text);
        let mut editor = editor_at(&source, offset);
        assert!(editor.insert_block(&mut source, "---", eol, None).unwrap());
        (source.to_string(), editor.cursor())
    }

    fn negrita_en(text: &str, offset: usize) -> (String, String) {
        let mut source = buffer(text);
        let mut editor = editor_at(&source, offset);
        assert!(editor.surround(&mut source, "**", "**", "texto").unwrap());
        let selected = source.slice_bytes(editor.selection()).unwrap();
        (source.to_string(), selected)
    }

    #[test]
    fn el_formato_sin_seleccion_rodea_la_palabra_bajo_el_cursor() {
        for (text, offset, expected, selected) in [
            ("resumen", 4, "**resumen**", "resumen"),
            ("resumen", 0, "**texto**resumen", "texto"),
            ("resumen", 7, "**resumen**", "resumen"),
            ("ab cd", 2, "**ab** cd", "ab"),
            ("ab cd", 3, "ab **texto**cd", "texto"),
            ("ab cd", 5, "ab **cd**", "cd"),
            ("canción ñandú", 3, "**canción** ñandú", "canción"),
            ("😀hola", "😀".len(), "😀**texto**hola", "texto"),
            ("😀hola", "😀hola".len(), "😀**hola**", "hola"),
            ("snake_case x", 3, "**snake_case** x", "snake_case"),
            ("a  b", 2, "a **texto** b", "texto"),
            ("", 0, "**texto**", "texto"),
            (
                "linea uno\nlinea dos",
                15,
                "linea uno\n**linea** dos",
                "linea",
            ),
        ] {
            let (result, chosen) = negrita_en(text, offset);
            assert_eq!(result, expected, "{text:?} en {offset}");
            assert_eq!(chosen, selected, "{text:?} en {offset}");
        }
    }

    #[test]
    fn el_formato_con_seleccion_no_cambia_de_comportamiento() {
        let mut source = buffer("una frase larga");
        let mut editor = editor_at(&source, 4);
        editor.set_cursor(&source, 9, true).unwrap();
        assert!(editor.surround(&mut source, "**", "**", "texto").unwrap());
        assert_eq!(source.to_string(), "una **frase** larga");
        assert!(editor.undo(&mut source).unwrap());
        assert_eq!(source.to_string(), "una frase larga");
    }

    #[test]
    fn un_bloque_queda_aislado_con_lineas_en_blanco() {
        // (documento, posición del cursor, resultado esperado)
        for (text, offset, expected) in [
            ("", 0, "---"),
            ("texto", 5, "texto\n\n---"),
            ("texto\n", 6, "texto\n\n---"),
            ("texto\n\n", 7, "texto\n\n---"),
            ("ab\ncd", 2, "ab\n\n---\n\ncd"),
            ("ab|cd", 3, "ab|cd\n\n---"),
            ("texto", 0, "---\n\ntexto"),
            ("uno\ndos", 5, "uno\ndos\n\n---"),
            ("a\n\nb", 2, "a\n\n---\n\nb"),
            ("- uno\n- dos", 11, "- uno\n- dos\n\n---"),
            ("\ntexto", 0, "---\n\ntexto"),
        ] {
            let (result, _) = insertar_bloque(text, offset, "\n");
            assert_eq!(result, expected, "{text:?} en {offset}");
        }
    }

    #[test]
    fn un_bloque_no_reemplaza_la_seleccion_ni_parte_la_linea() {
        let mut source = buffer("lo **nucleo** de la celula");
        let mut editor = editor_at(&source, 6);
        editor.set_cursor(&source, 12, true).unwrap();
        assert!(editor.insert_block(&mut source, "---", "\n", None).unwrap());
        assert_eq!(source.to_string(), "lo **nucleo** de la celula\n\n---");
    }

    #[test]
    fn un_bloque_con_el_cursor_al_inicio_de_una_linea_con_texto_va_encima() {
        let (result, cursor) = insertar_bloque("a\nuno\nb", 2, "\n");
        assert_eq!(result, "a\n\n---\n\nuno\nb");
        assert_eq!(cursor, "a\n\n---".len());
    }

    #[test]
    fn un_bloque_conserva_crlf_y_deja_el_cursor_al_final_del_bloque() {
        let (result, cursor) = insertar_bloque("texto", 5, "\r\n");
        assert_eq!(result, "texto\r\n\r\n---");
        assert_eq!(cursor, result.len());

        let (result, cursor) = insertar_bloque("ab\r\ncd", 2, "\r\n");
        assert_eq!(result, "ab\r\n\r\n---\r\n\r\ncd");
        assert_eq!(cursor, "ab\r\n\r\n---".len());
    }

    #[test]
    fn el_bloque_de_codigo_envuelve_la_seleccion_o_deja_un_texto_guia() {
        let mut source = buffer("antes");
        let mut editor = editor_at(&source, 5);
        assert!(editor.insert_code_block(&mut source, "\n").unwrap());
        assert_eq!(source.to_string(), "antes\n\n```\ncódigo\n```");
        let selected = source.slice_bytes(editor.selection()).unwrap();
        assert_eq!(selected, "código");

        let mut source = buffer("let x = 1;");
        let mut editor = editor_at(&source, 0);
        editor
            .set_cursor(&source, source.len_bytes(), true)
            .unwrap();
        assert!(editor.insert_code_block(&mut source, "\n").unwrap());
        assert_eq!(source.to_string(), "```\nlet x = 1;\n```");
        assert!(editor.undo(&mut source).unwrap());
        assert_eq!(source.to_string(), "let x = 1;");
    }

    #[test]
    fn enter_conserva_crlf_y_no_actua_dentro_del_marcador() {
        let mut source = buffer("1. uno\r\n");
        let mut editor = editor_at(&source, "1. uno".len());
        editor.insert_newline(&mut source, "\r\n").unwrap();
        assert_eq!(source.to_string(), "1. uno\r\n2. \r\n");

        let mut source = buffer("- item");
        let mut editor = editor_at(&source, 1);
        editor.insert_newline(&mut source, "\n").unwrap();
        assert_eq!(source.to_string(), "-\n item");
    }

    #[test]
    fn lista_sobre_parte_de_un_parrafo_separa_solo_la_seleccion() {
        let text = "Antes de la idea clave y después.";
        let mut source = buffer(text);
        let start = text.find("idea clave").unwrap();
        let mut editor = editor_at(&source, start);
        editor
            .set_cursor(&source, start + "idea clave".len(), true)
            .unwrap();
        assert!(
            editor
                .apply_line_prefix(&mut source, LinePrefix::Ordered, "\n")
                .unwrap()
        );
        assert_eq!(
            source.to_string(),
            "Antes de la \n1. idea clave\n y después."
        );
        assert_eq!(
            source.slice_bytes(editor.selection()).unwrap(),
            "1. idea clave"
        );
        assert!(editor.undo(&mut source).unwrap());
        assert_eq!(source.to_string(), text);
    }

    #[test]
    fn lista_sobre_varias_lineas_numera_cada_linea_con_texto() {
        let text = "uno\n\n  dos\r\ntres\nfuera";
        let mut source = buffer(text);
        let mut editor = editor_at(&source, 0);
        editor
            .set_cursor(&source, text.find("fuera").unwrap(), true)
            .unwrap();
        editor
            .apply_line_prefix(&mut source, LinePrefix::Ordered, "\n")
            .unwrap();
        assert_eq!(source.to_string(), "1. uno\n\n  2. dos\r\n3. tres\nfuera");
    }

    #[test]
    fn lista_sin_seleccion_prefija_la_linea_o_empieza_un_elemento() {
        let mut source = buffer("párrafo escrito");
        let mut editor = editor_at(&source, 3);
        editor
            .apply_line_prefix(&mut source, LinePrefix::Ordered, "\n")
            .unwrap();
        assert_eq!(source.to_string(), "1. párrafo escrito");

        let mut source = buffer("");
        let mut editor = editor_at(&source, 0);
        editor
            .apply_line_prefix(&mut source, LinePrefix::Task, "\n")
            .unwrap();
        assert_eq!(source.to_string(), "- [ ] ");
        assert_eq!(editor.cursor(), "- [ ] ".len());
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
    fn prefijar_linea_con_crlf_conserva_el_resto_y_la_seleccion() {
        let mut source = buffer("primera\r\nidea ágil\r\nfinal");
        let mut editor = SourceEditor::new();
        let start = "primera\r\n".len();
        editor.set_cursor(&source, start, false).unwrap();
        editor
            .set_cursor(&source, start + "idea ágil".len(), true)
            .unwrap();

        assert!(editor.prefix_current_line(&mut source, "- ").unwrap());
        assert_eq!(source.to_string(), "primera\r\n- idea ágil\r\nfinal");
        assert_eq!(source.slice_bytes(editor.selection()).unwrap(), "idea ágil");
        assert!(editor.undo(&mut source).unwrap());
        assert_eq!(source.to_string(), "primera\r\nidea ágil\r\nfinal");
    }

    #[test]
    fn prefijar_linea_no_mueve_un_ancla_en_linea_anterior() {
        let mut source = buffer("primera\nsegunda\ntercera");
        let mut editor = SourceEditor::new();
        editor.set_cursor(&source, 0, false).unwrap();
        editor
            .set_cursor(&source, "primera\nsegunda".len(), true)
            .unwrap();

        assert!(editor.prefix_current_line(&mut source, "## ").unwrap());
        assert_eq!(source.to_string(), "primera\n## segunda\ntercera");
        assert_eq!(editor.anchor(), 0);
        assert_eq!(editor.cursor(), "primera\n## segunda".len());
    }

    #[test]
    fn cambiar_nivel_de_encabezado_no_acumula_marcadores_y_es_reversible() {
        let mut source = buffer("intro\r\n  ## Título ágil\r\nfinal");
        let mut editor = SourceEditor::new();
        let content_start = "intro\r\n  ## ".len();
        editor.set_cursor(&source, content_start, false).unwrap();
        editor
            .set_cursor(&source, content_start + "Título ágil".len(), true)
            .unwrap();

        assert!(editor.set_heading_level(&mut source, 4).unwrap());
        assert_eq!(source.to_string(), "intro\r\n  #### Título ágil\r\nfinal");
        assert_eq!(
            source.slice_bytes(editor.selection()).unwrap(),
            "Título ágil"
        );
        assert!(editor.undo(&mut source).unwrap());
        assert_eq!(source.to_string(), "intro\r\n  ## Título ágil\r\nfinal");

        let mut plain = buffer("sin encabezado");
        let mut plain_editor = SourceEditor::new();
        plain_editor
            .set_cursor(&plain, plain.len_bytes(), false)
            .unwrap();
        assert!(plain_editor.set_heading_level(&mut plain, 1).unwrap());
        assert_eq!(plain.to_string(), "# sin encabezado");
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
    fn crlf_es_atomico_para_cursor_seleccion_y_borrado() {
        let mut source = buffer("a\r\nb");
        let mut editor = SourceEditor::new();

        assert!(source.is_edit_boundary(1));
        assert!(!source.is_edit_boundary(2));
        assert!(source.is_edit_boundary(3));
        assert_eq!(
            editor.set_cursor(&source, 2, false),
            Err(EditError::InvalidRange)
        );
        assert_eq!(source.slice_bytes(1..2), Err(EditError::InvalidRange));

        editor.set_cursor(&source, 1, false).unwrap();
        editor.move_right(&source, false).unwrap();
        assert_eq!(editor.cursor(), 3);
        editor.move_left(&source, true).unwrap();
        assert_eq!(editor.selection(), 1..3);
        assert_eq!(
            editor.selected_text(&source).unwrap().as_deref(),
            Some("\r\n")
        );

        editor.set_cursor(&source, 1, false).unwrap();
        assert!(editor.delete(&mut source).unwrap());
        assert_eq!(source.to_string(), "ab");
        assert!(editor.undo(&mut source).unwrap());
        assert_eq!(source.to_string(), "a\r\nb");

        editor.set_cursor(&source, 3, false).unwrap();
        assert!(editor.backspace(&mut source).unwrap());
        assert_eq!(source.to_string(), "ab");
        assert!(editor.undo(&mut source).unwrap());
        assert_eq!(source.to_string(), "a\r\nb");
    }

    #[test]
    fn historial_puede_revertir_un_crlf_formado_en_el_borde_de_un_parche() {
        let mut source = buffer("\n");
        let mut editor = SourceEditor::new();
        editor.set_cursor(&source, 0, false).unwrap();

        assert!(editor.insert(&mut source, "\r").unwrap());
        assert_eq!(source.to_string(), "\r\n");
        assert_eq!(editor.cursor(), 2);
        assert!(editor.undo(&mut source).unwrap());
        assert_eq!(source.to_string(), "\n");
        assert_eq!(editor.cursor(), 0);
        assert!(editor.redo(&mut source).unwrap());
        assert_eq!(source.to_string(), "\r\n");
        assert_eq!(editor.cursor(), 2);
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
        assert_eq!(editor.cursor(), "áéí\n文\n🔒🔒".len());
    }

    #[test]
    fn navegacion_vertical_conserva_columna_de_caracteres_no_de_bytes() {
        let source = buffer("áéí\nabcde\n日本語");
        let mut editor = SourceEditor::new();
        editor.set_cursor(&source, "áé".len(), false).unwrap();

        editor.move_line(&source, true, false).unwrap();
        assert_eq!(editor.cursor(), "áéí\nab".len());
        editor.move_line(&source, true, false).unwrap();
        assert_eq!(editor.cursor(), "áéí\nabcde\n日本".len());
    }

    #[test]
    fn inicio_y_fin_respetan_crlf_y_seleccion() {
        let source = buffer("uno\r\ndos\r\n");
        let mut editor = SourceEditor::new();
        editor.set_cursor(&source, "uno\r\nd".len(), false).unwrap();
        editor.move_line_boundary(&source, false, false).unwrap();
        assert_eq!(editor.cursor(), "uno\r\n".len());
        editor.move_line_boundary(&source, true, true).unwrap();
        assert_eq!(editor.selection(), "uno\r\n".len().."uno\r\ndos".len());
    }

    #[test]
    fn inicio_y_fin_de_documento_conservan_la_seleccion_unicode() {
        let source = buffer("inicio á\r\nfinal 🔒");
        let mut editor = SourceEditor::new();
        editor
            .set_cursor(&source, "inicio á\r\n".len(), false)
            .unwrap();

        editor.move_document_boundary(&source, true, true).unwrap();
        assert_eq!(editor.selection(), "inicio á\r\n".len()..source.len_bytes());

        editor
            .move_document_boundary(&source, false, false)
            .unwrap();
        assert_eq!(editor.cursor(), 0);
        assert_eq!(editor.anchor(), 0);
    }

    #[test]
    fn navegacion_por_palabras_respeta_unicode_crlf_y_seleccion() {
        let source = buffer("uno áé, dos\r\n🔒_clave final");
        let mut editor = SourceEditor::new();
        editor.set_cursor(&source, 0, false).unwrap();

        editor.move_word(&source, true, false).unwrap();
        assert_eq!(editor.cursor(), "uno ".len());
        editor.move_word(&source, true, true).unwrap();
        assert_eq!(editor.selection(), "uno ".len().."uno áé, ".len());

        editor.move_word(&source, true, false).unwrap();
        assert_eq!(editor.cursor(), "uno áé, dos\r\n🔒".len());
        editor.move_word(&source, false, true).unwrap();
        assert_eq!(
            editor.selection(),
            "uno áé, ".len().."uno áé, dos\r\n🔒".len()
        );
    }

    #[test]
    fn copia_la_seleccion_de_fuente_unicode_sin_consultar_el_renderer() {
        let source = buffer("antes á🔒 después");
        let mut editor = SourceEditor::new();
        editor.set_cursor(&source, "antes ".len(), false).unwrap();
        editor.set_cursor(&source, "antes á🔒".len(), true).unwrap();

        assert_eq!(
            editor.selected_text(&source).unwrap(),
            Some("á🔒".to_owned())
        );
        editor.set_cursor(&source, 0, false).unwrap();
        assert_eq!(editor.selected_text(&source).unwrap(), None);
    }

    #[test]
    fn fin_de_linea_crlf_no_deja_el_cursor_entre_los_dos_bytes() {
        let source = buffer("uno\r\ndos");
        let mut editor = SourceEditor::new();
        editor.set_cursor(&source, 0, false).unwrap();

        editor.move_line_boundary(&source, true, false).unwrap();
        assert_eq!(editor.cursor(), "uno".len());
    }

    #[test]
    fn seleccionar_todo_abarca_la_fuente_utf8_completa() {
        let source = buffer("á\n🔒");
        let mut editor = SourceEditor::new();
        editor.select_all(&source);
        assert_eq!(editor.selection(), 0..source.len_bytes());
    }

    #[test]
    fn borrar_una_seleccion_unicode_es_reversible() {
        let mut source = buffer("antes 日本語 después");
        let mut editor = SourceEditor::new();
        let start = "antes ".len();
        let end = start + "日本語 ".len();
        editor.set_cursor(&source, start, false).unwrap();
        editor.set_cursor(&source, end, true).unwrap();

        assert!(editor.insert(&mut source, "").unwrap());
        assert_eq!(source.to_string(), "antes después");
        assert!(editor.undo(&mut source).unwrap());
        assert_eq!(source.to_string(), "antes 日本語 después");
    }

    #[test]
    fn deshacer_y_rehacer_reubican_el_cursor_en_el_cambio() {
        let mut source = buffer("uno dos");
        let mut editor = SourceEditor::new();
        editor.set_cursor(&source, "uno ".len(), false).unwrap();
        editor.insert(&mut source, "🔐").unwrap();

        assert!(editor.undo(&mut source).unwrap());
        assert_eq!(source.to_string(), "uno dos");
        assert_eq!(editor.cursor(), "uno ".len());
        assert!(editor.redo(&mut source).unwrap());
        assert_eq!(source.to_string(), "uno 🔐dos");
        assert_eq!(editor.cursor(), "uno 🔐".len());
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
