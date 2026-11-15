//! Desplazamiento de las listas de paneles y menús. La fila elegida y las
//! filas visibles son cosas distintas: la rueda desplaza la ventana sin
//! cambiar la selección, y las flechas mueven la selección y solo desplazan la
//! ventana lo mínimo para que no se salga de la vista. Es puro: no dibuja ni
//! conoce paneles concretos.

use std::ops::Range;

/// Clave de la lista abierta. Al cambiar (otro panel, otro menú) el
/// desplazamiento empieza de cero.
pub(crate) type ListKey = Option<u32>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ListNav {
    PageUp,
    PageDown,
    Home,
    End,
}

/// Fila elegida tras una tecla de navegación larga. No da la vuelta en los
/// extremos: Fin en la última fila se queda ahí.
pub(crate) fn navigate(selected: usize, total: usize, capacity: usize, nav: ListNav) -> usize {
    if total == 0 {
        return 0;
    }
    let last = total - 1;
    let page = capacity.saturating_sub(1).max(1);
    match nav {
        ListNav::Home => 0,
        ListNav::End => last,
        ListNav::PageDown => (selected + page).min(last),
        ListNav::PageUp => selected.saturating_sub(page),
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct ListScroll {
    top: usize,
    /// Última fila elegida vista. Si cambió, la ventana se reajusta para
    /// mostrarla; si no, el desplazamiento manual se respeta.
    seen: Option<usize>,
    key: ListKey,
}

impl ListScroll {
    /// Primera fila visible para el estado dado, sin modificar nada.
    fn resolved_top(&self, key: ListKey, total: usize, selected: usize, capacity: usize) -> usize {
        let max_top = total.saturating_sub(capacity);
        if capacity == 0 || total == 0 {
            return 0;
        }
        let selected = selected.min(total - 1);
        let (top, follow) = if self.key != key {
            (0, true)
        } else {
            (self.top, self.seen != Some(selected))
        };
        let top = top.min(max_top);
        if !follow {
            return top;
        }
        if selected < top {
            selected
        } else if selected >= top + capacity {
            selected + 1 - capacity
        } else {
            top
        }
        .min(max_top)
    }

    /// Filas visibles sin modificar el estado: sirve para el clic y el
    /// dibujo de una fila ya sincronizada.
    pub(crate) fn range(
        &self,
        key: ListKey,
        total: usize,
        selected: usize,
        capacity: usize,
    ) -> Range<usize> {
        let top = self.resolved_top(key, total, selected, capacity);
        top..(top + capacity).min(total)
    }

    /// Igual que `range`, pero recuerda el resultado.
    pub(crate) fn sync(
        &mut self,
        key: ListKey,
        total: usize,
        selected: usize,
        capacity: usize,
    ) -> Range<usize> {
        let top = self.resolved_top(key, total, selected, capacity);
        self.top = top;
        self.key = key;
        self.seen = (total > 0).then(|| selected.min(total - 1));
        top..(top + capacity).min(total)
    }

    /// Desplaza la ventana `rows` filas (positivo baja) sin tocar la
    /// selección. Devuelve `true` si se movió.
    pub(crate) fn scroll_rows(
        &mut self,
        key: ListKey,
        rows: isize,
        total: usize,
        selected: usize,
        capacity: usize,
    ) -> bool {
        let before = self.resolved_top(key, total, selected, capacity);
        let max_top = total.saturating_sub(capacity);
        let moved = (before as isize + rows).clamp(0, max_top as isize) as usize;
        self.top = moved;
        self.key = key;
        self.seen = (total > 0).then(|| selected.min(total - 1));
        moved != before
    }

    /// Posición y alto del indicador de desplazamiento como fracciones de la
    /// altura de la lista, o `None` si todo entra y no hace falta.
    pub(crate) fn thumb(total: usize, range: &Range<usize>) -> Option<(f32, f32)> {
        let visible = range.len();
        if total == 0 || visible == 0 || visible >= total {
            return None;
        }
        let height = visible as f32 / total as f32;
        let start = range.start as f32 / total as f32;
        Some((start, height))
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    const KEY: ListKey = Some(1);

    #[test]
    fn la_ventana_sigue_a_la_seleccion_con_el_minimo_desplazamiento() {
        let mut scroll = ListScroll::default();
        assert_eq!(scroll.sync(KEY, 30, 0, 9), 0..9);
        assert_eq!(scroll.sync(KEY, 30, 8, 9), 0..9);
        // Un paso más allá del borde inferior baja una sola fila.
        assert_eq!(scroll.sync(KEY, 30, 9, 9), 1..10);
        assert_eq!(scroll.sync(KEY, 30, 10, 9), 2..11);
        // Subir dentro de la ventana no la mueve.
        assert_eq!(scroll.sync(KEY, 30, 5, 9), 2..11);
        // Pasar el borde superior sube una sola fila.
        assert_eq!(scroll.sync(KEY, 30, 1, 9), 1..10);
        assert_eq!(scroll.sync(KEY, 30, 29, 9), 21..30);
    }

    #[test]
    fn la_rueda_desplaza_sin_cambiar_la_seleccion_y_se_respeta() {
        let mut scroll = ListScroll::default();
        scroll.sync(KEY, 30, 2, 9);
        assert!(scroll.scroll_rows(KEY, 6, 30, 2, 9));
        // La selección quedó fuera de la vista y la ventana no se reajusta.
        assert_eq!(scroll.range(KEY, 30, 2, 9), 6..15);
        assert_eq!(scroll.sync(KEY, 30, 2, 9), 6..15);
        // Hasta que la selección cambia: ahí vuelve a mostrarse.
        assert_eq!(scroll.sync(KEY, 30, 3, 9), 3..12);
    }

    #[test]
    fn la_rueda_no_pasa_de_los_extremos() {
        let mut scroll = ListScroll::default();
        scroll.sync(KEY, 30, 0, 9);
        assert!(!scroll.scroll_rows(KEY, -3, 30, 0, 9));
        assert!(scroll.scroll_rows(KEY, 100, 30, 0, 9));
        assert_eq!(scroll.range(KEY, 30, 0, 9), 21..30);
        assert!(!scroll.scroll_rows(KEY, 5, 30, 0, 9));
        assert!(scroll.scroll_rows(KEY, -100, 30, 0, 9));
        assert_eq!(scroll.range(KEY, 30, 0, 9), 0..9);
    }

    #[test]
    fn otra_lista_empieza_desde_el_principio() {
        let mut scroll = ListScroll::default();
        scroll.sync(KEY, 30, 20, 9);
        assert_eq!(scroll.range(KEY, 30, 20, 9), 12..21);
        assert_eq!(scroll.sync(Some(2), 30, 0, 9), 0..9);
    }

    #[test]
    fn los_bordes_con_cero_uno_y_pocos_elementos() {
        let mut scroll = ListScroll::default();
        assert_eq!(scroll.sync(KEY, 0, 0, 9), 0..0);
        assert_eq!(scroll.sync(KEY, 1, 0, 9), 0..1);
        assert_eq!(scroll.sync(KEY, 3, 2, 9), 0..3);
        assert_eq!(scroll.sync(KEY, 5, 4, 0), 0..0);
        assert!(!scroll.scroll_rows(KEY, 3, 3, 0, 9));
        // La lista se acorta (filtro): la ventana no se pasa del final.
        scroll.sync(KEY, 30, 29, 9);
        assert_eq!(scroll.sync(KEY, 12, 5, 9), 3..12);
    }

    #[test]
    fn range_no_modifica_el_estado() {
        let mut scroll = ListScroll::default();
        scroll.sync(KEY, 30, 0, 9);
        let antes = scroll;
        assert_eq!(scroll.range(KEY, 30, 25, 9), 17..26);
        assert_eq!(scroll, antes);
    }

    #[test]
    fn las_teclas_largas_no_dan_la_vuelta() {
        assert_eq!(navigate(0, 30, 9, ListNav::PageDown), 8);
        assert_eq!(navigate(8, 30, 9, ListNav::PageDown), 16);
        assert_eq!(navigate(25, 30, 9, ListNav::PageDown), 29);
        assert_eq!(navigate(29, 30, 9, ListNav::PageDown), 29);
        assert_eq!(navigate(5, 30, 9, ListNav::PageUp), 0);
        assert_eq!(navigate(0, 30, 9, ListNav::PageUp), 0);
        assert_eq!(navigate(12, 30, 9, ListNav::Home), 0);
        assert_eq!(navigate(12, 30, 9, ListNav::End), 29);
        assert_eq!(navigate(0, 0, 9, ListNav::End), 0);
        assert_eq!(navigate(0, 3, 1, ListNav::PageDown), 1);
    }

    #[test]
    fn el_indicador_refleja_la_ventana_y_no_aparece_si_todo_entra() {
        assert_eq!(ListScroll::thumb(9, &(0..9)), None);
        assert_eq!(ListScroll::thumb(0, &(0..0)), None);
        let (inicio, alto) = ListScroll::thumb(30, &(15..24)).unwrap();
        assert!((inicio - 0.5).abs() < 1e-6);
        assert!((alto - 0.3).abs() < 1e-6);
    }
}
