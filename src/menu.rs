//! Entradas de un menú desplegable. Un menú no es solo una lista de acciones:
//! los encabezados agrupan («Títulos», «Bloques») y no se pueden elegir. La
//! navegación con flechas, Re Pág, Inicio y Fin salta sobre ellos.

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MenuEntry {
    Item(AppAction),
    /// Rótulo de un grupo. No se selecciona ni se pulsa.
    Heading(&'static str),
}

impl MenuEntry {
    pub(crate) fn action(self) -> Option<AppAction> {
        match self {
            Self::Item(action) => Some(action),
            Self::Heading(_) => None,
        }
    }

    pub(crate) fn is_heading(self) -> bool {
        matches!(self, Self::Heading(_))
    }
}

/// Entradas planas, sin encabezados, para una lista de acciones.
pub(crate) fn items(actions: &[AppAction]) -> Vec<MenuEntry> {
    actions.iter().copied().map(MenuEntry::Item).collect()
}

/// Posición elegible más cercana a `index`: la propia, la siguiente o, si no
/// hay ninguna después, la anterior. Con un menú sin elegibles devuelve 0.
pub(crate) fn snap(entries: &[MenuEntry], index: usize) -> usize {
    let index = index.min(entries.len().saturating_sub(1));
    (index..entries.len())
        .chain((0..index).rev())
        .find(|candidate| !entries[*candidate].is_heading())
        .unwrap_or(0)
}

/// Siguiente (o anterior) posición elegible, dando la vuelta al llegar al
/// extremo. Con un menú sin elegibles se queda donde está.
pub(crate) fn step(entries: &[MenuEntry], from: usize, forward: bool) -> usize {
    let count = entries.len();
    if count == 0 {
        return 0;
    }
    (1..=count)
        .map(|offset| {
            if forward {
                (from + offset) % count
            } else {
                (from + count * offset - offset) % count
            }
        })
        .find(|candidate| !entries[*candidate].is_heading())
        .unwrap_or(from)
}

#[cfg(test)]
mod pruebas {
    use super::*;

    fn menu() -> Vec<MenuEntry> {
        vec![
            MenuEntry::Heading("Uno"),
            MenuEntry::Item(AppAction::Save),
            MenuEntry::Item(AppAction::SaveAs),
            MenuEntry::Heading("Dos"),
            MenuEntry::Item(AppAction::ExportPdf),
        ]
    }

    #[test]
    fn el_encabezado_no_se_elige_y_la_seleccion_se_corre_al_siguiente_elemento() {
        let entradas = menu();
        assert_eq!(snap(&entradas, 0), 1);
        assert_eq!(snap(&entradas, 3), 4);
        assert_eq!(snap(&entradas, 2), 2);
        // Un índice fuera de rango queda dentro.
        assert_eq!(snap(&entradas, 99), 4);
        // Sin nada elegible después, se retrocede.
        let mut final_con_encabezado = menu();
        final_con_encabezado.push(MenuEntry::Heading("Fin"));
        assert_eq!(snap(&final_con_encabezado, 5), 4);
        assert_eq!(snap(&[], 0), 0);
    }

    #[test]
    fn las_flechas_saltan_los_encabezados_y_dan_la_vuelta() {
        let entradas = menu();
        assert_eq!(step(&entradas, 1, true), 2);
        assert_eq!(step(&entradas, 2, true), 4);
        assert_eq!(step(&entradas, 4, true), 1);
        assert_eq!(step(&entradas, 1, false), 4);
        assert_eq!(step(&entradas, 4, false), 2);
        assert_eq!(step(&entradas, 2, false), 1);
        // Un menú de un solo elemento no se mueve; uno vacío tampoco.
        let solo = [MenuEntry::Heading("X"), MenuEntry::Item(AppAction::Save)];
        assert_eq!(step(&solo, 1, true), 1);
        assert_eq!(step(&solo, 1, false), 1);
        assert_eq!(step(&[], 0, true), 0);
    }
}
