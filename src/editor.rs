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
}
