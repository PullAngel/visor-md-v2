//! La lista que está abierta en un panel o menú, vista como una sola cosa:
//! cuántas filas tiene, cuál está elegida, cuántas caben y cuáles se ven. El
//! dibujo, el clic, la rueda y las teclas largas pasan por aquí para no
//! desincronizarse.

use super::*;
use crate::list_view::{ListKey, ListNav, navigate};

/// Tope de filas de un panel flotante, aunque la ventana dé más lugar: un menú
/// que ocupa toda la ventana deja de ser un menú.
const MAX_FLOATING_ROWS: usize = 14;

/// Filas que caben en un panel flotante según el alto de la ventana.
pub(crate) fn floating_capacity(window_height: f32, compact: bool) -> usize {
    let title_and_footer = if compact {
        0.0
    } else {
        PANEL_ROW_HEIGHT + PANEL_FOOTER_HEIGHT
    };
    let overhead = FLOATING_PANEL_Y + STATUS_HEIGHT + 12.0 + PANEL_PADDING * 2.0 + title_and_footer;
    let rows = ((window_height - overhead) / PANEL_ROW_HEIGHT)
        .floor()
        .max(1.0) as usize;
    rows.min(MAX_FLOATING_ROWS)
}

/// Filas que muestra un panel flotante con `total` elementos.
pub(crate) fn floating_rows(total: usize, window_height: f32, compact: bool) -> usize {
    total.min(floating_capacity(window_height, compact))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct OpenList {
    pub(crate) total: usize,
    pub(crate) selected: usize,
}

/// Tecla de navegación larga (Re Pág, Av Pág, Inicio, Fin) sin Ctrl ni Alt.
pub(crate) fn list_nav_for_key(event: &KeyEvent, modified: bool) -> Option<ListNav> {
    if event.state != ElementState::Pressed || modified {
        return None;
    }
    match event.physical_key {
        PhysicalKey::Code(KeyCode::PageUp) => Some(ListNav::PageUp),
        PhysicalKey::Code(KeyCode::PageDown) => Some(ListNav::PageDown),
        PhysicalKey::Code(KeyCode::Home) => Some(ListNav::Home),
        PhysicalKey::Code(KeyCode::End) => Some(ListNav::End),
        _ => None,
    }
}

impl App {
    /// Lista abierta con su tamaño y su fila elegida, si hay alguna.
    pub(crate) fn open_list(&self) -> Option<OpenList> {
        let (total, selected) = if self.pane_split_picker.is_some() {
            (self.pane_split_candidates.len(), self.pane_split_match)
        } else if let Some(selected) = self.symbol_picker {
            (symbol_matches(&self.symbol_query).len(), selected)
        } else if let Some(selected) = self.command_palette {
            let entries = self.command_palette_entries();
            (entries.len(), menu::snap(&entries, selected))
        } else if self.workspace_search_query.is_some() {
            (
                self.workspace_search_matches().len(),
                self.workspace_search_match,
            )
        } else if let Some(paths) = &self.workspace_paths {
            (paths.len(), self.workspace_path_match)
        } else if let Some(paths) = &self.backlink_paths {
            (paths.len(), self.backlink_match)
        } else if let Some(rows) = &self.link_diagnostics {
            (rows.len(), self.link_diagnostic_match)
        } else if let Some(headings) = &self.outline_headings {
            (headings.len(), self.outline_match)
        } else {
            return None;
        };
        Some(OpenList { total, selected })
    }

    fn set_open_list_selected(&mut self, index: usize) {
        if self.pane_split_picker.is_some() {
            self.pane_split_match = index;
        } else if self.symbol_picker.is_some() {
            self.symbol_picker = Some(index);
        } else if self.command_palette.is_some() {
            let entries = self.command_palette_entries();
            self.command_palette = Some(menu::snap(&entries, index));
        } else if self.workspace_search_query.is_some() {
            self.workspace_search_match = index;
        } else if self.workspace_paths.is_some() {
            self.workspace_path_match = index;
        } else if self.backlink_paths.is_some() {
            self.backlink_match = index;
        } else if self.link_diagnostics.is_some() {
            self.link_diagnostic_match = index;
        } else if self.outline_headings.is_some() {
            self.outline_match = index;
        }
    }

    /// Identifica la lista abierta; al cambiar, el desplazamiento reinicia.
    pub(crate) fn list_key(&self) -> ListKey {
        if self.pane_split_picker.is_some() {
            Some(10_001)
        } else if self.symbol_picker.is_some() {
            Some(10_002)
        } else {
            self.open_surface().map(|action| action as u32)
        }
    }

    /// Filas que caben en el panel abierto con `total` elementos.
    pub(crate) fn list_capacity(&self, total: usize) -> usize {
        self.navigation_panel_geometry(total)
            .map_or(0, panel_capacity)
    }

    /// Índice del elemento bajo el punto, con el desplazamiento actual.
    pub(crate) fn list_item_at(
        &self,
        x: f32,
        y: f32,
        total: usize,
        selected: usize,
    ) -> Option<usize> {
        let geometry = self.navigation_panel_geometry(total)?;
        let range =
            self.panel_scroll
                .range(self.list_key(), total, selected, panel_capacity(geometry));
        panel_item_at(x, y, range, geometry)
    }

    /// La rueda desplaza la lista abierta sin cambiar su selección.
    pub(crate) fn scroll_open_list(&mut self, rows: isize) -> bool {
        let Some(list) = self.open_list() else {
            return false;
        };
        let capacity = self.list_capacity(list.total);
        let key = self.list_key();
        self.panel_scroll
            .scroll_rows(key, rows, list.total, list.selected, capacity)
    }

    /// Re Pág, Av Pág, Inicio y Fin sobre la lista abierta.
    pub(crate) fn navigate_open_list(&mut self, nav: ListNav) -> bool {
        let Some(list) = self.open_list() else {
            return false;
        };
        let capacity = self.list_capacity(list.total);
        self.set_open_list_selected(navigate(list.selected, list.total, capacity, nav));
        true
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn un_panel_flotante_muestra_mas_filas_si_hay_lugar_y_nunca_mas_de_catorce() {
        assert!(floating_capacity(480.0, false) >= 7);
        assert!(floating_capacity(480.0, false) < floating_capacity(720.0, false));
        assert_eq!(floating_capacity(1_400.0, false), MAX_FLOATING_ROWS);
        assert_eq!(floating_rows(3, 720.0, false), 3);
        assert_eq!(
            floating_rows(69, 720.0, false),
            floating_capacity(720.0, false)
        );
    }

    #[test]
    fn la_paleta_abierta_es_la_lista_y_la_rueda_no_cambia_su_seleccion() {
        let mut app = App::for_tests(NIGHT);
        app.load_for_test("# Uno\n", DocumentMode::Reading);
        app.draw_for_test(900, 560);
        app.open_command_palette();
        let lista = app.open_list().expect("la paleta es una lista");
        assert!(lista.total > 30);
        assert_eq!(lista.selected, 0);
        // La capacidad sale de la geometría del panel.
        let capacidad = app.list_capacity(lista.total);
        assert_eq!(capacidad, floating_capacity(560.0, false));

        assert!(app.scroll_open_list(5));
        assert_eq!(app.open_list().unwrap().selected, 0);
        let rango = app
            .panel_scroll
            .range(app.list_key(), lista.total, 0, capacidad);
        assert_eq!(rango.start, 5);
    }

    #[test]
    fn las_teclas_largas_mueven_la_seleccion_de_la_lista_abierta() {
        let mut app = App::for_tests(NIGHT);
        app.load_for_test("# Uno\n", DocumentMode::Reading);
        app.draw_for_test(900, 560);
        app.open_command_palette();
        let total = app.open_list().unwrap().total;
        let capacidad = app.list_capacity(total);
        assert!(app.navigate_open_list(ListNav::PageDown));
        assert_eq!(app.open_list().unwrap().selected, capacidad - 1);
        assert!(app.navigate_open_list(ListNav::End));
        assert_eq!(app.open_list().unwrap().selected, total - 1);
        assert!(app.navigate_open_list(ListNav::Home));
        assert_eq!(app.open_list().unwrap().selected, 0);
    }

    #[test]
    fn el_clic_sobre_una_fila_desplazada_elige_el_indice_correcto() {
        let mut app = App::for_tests(NIGHT);
        app.load_for_test("# Uno\n", DocumentMode::Reading);
        app.draw_for_test(900, 560);
        app.open_command_palette();
        let total = app.open_list().unwrap().total;
        app.scroll_open_list(7);
        let geometria = app.navigation_panel_geometry(total).unwrap();
        // La tercera fila visible es el elemento 7 + 2.
        let y = geometria.items_y + PANEL_ROW_HEIGHT * 2.5;
        assert_eq!(app.list_item_at(geometria.x + 20.0, y, total, 0), Some(9));
    }

    #[test]
    fn sin_lista_abierta_no_hay_nada_que_desplazar() {
        let mut app = App::for_tests(NIGHT);
        app.load_for_test("# Uno\n", DocumentMode::Reading);
        assert!(app.open_list().is_none());
        assert!(!app.scroll_open_list(3));
        assert!(!app.navigate_open_list(ListNav::End));
    }
}
