//! Paneles y menús que abre la barra, y su alternancia: un botón que abrió algo
//! lo cierra con el mismo clic; pulsar el botón de otro panel cambia directo,
//! sin pasar por un clic extra que solo cierre. Cerrar nunca toca el
//! documento.

use super::*;

/// Qué debe hacer el clic sobre un botón de la barra.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SurfacePress {
    /// La acción no abre un panel: se ejecuta sin tocar lo que haya abierto.
    Run,
    /// Cierra el panel abierto, que es el de este botón.
    Close,
    /// Cierra lo que haya abierto y abre el panel de este botón.
    Open,
}

/// Superficie que abre una acción de la barra. Es la propia acción, salvo el
/// espacio de trabajo: con una carpeta ya elegida abre sus archivos.
pub(crate) fn surface_for_action(action: AppAction, workspace_open: bool) -> Option<AppAction> {
    match action {
        AppAction::DocumentOutline
        | AppAction::WorkspaceFiles
        | AppAction::SearchWorkspace
        | AppAction::Backlinks
        | AppAction::SearchDocument
        | AppAction::CommandPalette
        | AppAction::BlockTools
        | AppAction::ListTools
        | AppAction::InsertTools
        | AppAction::StudyTools
        | AppAction::WritingTools
        | AppAction::MenuFile
        | AppAction::MenuView
        | AppAction::MenuFolder
        | AppAction::MenuAll => Some(action),
        AppAction::WorkspaceHub => Some(if workspace_open {
            AppAction::WorkspaceFiles
        } else {
            AppAction::WorkspaceHub
        }),
        _ => None,
    }
}

pub(crate) fn surface_press(
    open: Option<AppAction>,
    action: AppAction,
    workspace_open: bool,
) -> SurfacePress {
    match surface_for_action(action, workspace_open) {
        None => SurfacePress::Run,
        Some(surface) if open == Some(surface) => SurfacePress::Close,
        Some(_) => SurfacePress::Open,
    }
}

/// Un botón se ve activo si su modo está encendido o si su panel está abierto.
pub(crate) fn toolbar_button_active(
    action: AppAction,
    mode: DocumentMode,
    open: Option<AppAction>,
    workspace_open: bool,
) -> bool {
    toolbar_action_is_active(action, mode)
        || (open.is_some() && surface_for_action(action, workspace_open) == open)
}

impl App {
    /// Acción de la barra que abrió lo que está visible, si hay algo abierto.
    pub(crate) fn open_surface(&self) -> Option<AppAction> {
        if self.command_palette.is_some() {
            return Some(match self.command_palette_scope {
                CommandPaletteScope::All => AppAction::CommandPalette,
                CommandPaletteScope::File => AppAction::MenuFile,
                CommandPaletteScope::View => AppAction::MenuView,
                CommandPaletteScope::Folder => AppAction::MenuFolder,
                CommandPaletteScope::Menu => AppAction::MenuAll,
                CommandPaletteScope::Writing => AppAction::WritingTools,
                CommandPaletteScope::Blocks => AppAction::BlockTools,
                CommandPaletteScope::Lists => AppAction::ListTools,
                CommandPaletteScope::Insert => AppAction::InsertTools,
                CommandPaletteScope::Workspace => AppAction::WorkspaceHub,
                CommandPaletteScope::Study => AppAction::StudyTools,
            });
        }
        if self.outline_headings.is_some() {
            Some(AppAction::DocumentOutline)
        } else if self.workspace_paths.is_some() {
            Some(AppAction::WorkspaceFiles)
        } else if self.workspace_search_query.is_some() {
            Some(AppAction::SearchWorkspace)
        } else if self.backlink_paths.is_some() || self.link_diagnostics.is_some() {
            Some(AppAction::Backlinks)
        } else if self.search_query.is_some() {
            Some(AppAction::SearchDocument)
        } else {
            None
        }
    }

    /// Cierra el panel, menú o búsqueda abiertos. No toca el documento.
    pub(crate) fn close_open_surface(&mut self) {
        self.dismiss_navigation_panel();
        self.search_query = None;
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    /// Clic en un botón de la barra: alterna su panel o ejecuta su acción.
    pub(crate) fn press_toolbar_action(&mut self, action: AppAction) {
        let workspace_open = self.workspace.is_some();
        match surface_press(self.open_surface(), action, workspace_open) {
            SurfacePress::Run => self.perform_action(action),
            SurfacePress::Close => self.close_open_surface(),
            SurfacePress::Open => {
                self.close_open_surface();
                // El menú se abre justo debajo del botón que lo pidió.
                if let Some((width, _)) = self.view_size() {
                    self.panel_anchor = toolbar_button_x(action, width, self.document.mode);
                }
                self.perform_action(action);
            }
        }
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    const SUPERFICIES: [AppAction; 11] = [
        AppAction::DocumentOutline,
        AppAction::WorkspaceFiles,
        AppAction::SearchWorkspace,
        AppAction::Backlinks,
        AppAction::SearchDocument,
        AppAction::CommandPalette,
        AppAction::BlockTools,
        AppAction::ListTools,
        AppAction::InsertTools,
        AppAction::StudyTools,
        AppAction::WritingTools,
    ];

    #[test]
    fn el_mismo_boton_cierra_su_panel_y_otro_boton_cambia_directo() {
        for abierta in SUPERFICIES {
            for pulsada in SUPERFICIES {
                let esperado = if abierta == pulsada {
                    SurfacePress::Close
                } else {
                    SurfacePress::Open
                };
                assert_eq!(
                    surface_press(Some(abierta), pulsada, true),
                    esperado,
                    "{abierta:?} abierta, {pulsada:?} pulsada"
                );
            }
            // Sin nada abierto, pulsar un botón de panel lo abre.
            assert_eq!(surface_press(None, abierta, true), SurfacePress::Open);
        }
    }

    #[test]
    fn las_acciones_que_no_abren_panel_no_tocan_lo_abierto() {
        for abierta in [None, Some(AppAction::DocumentOutline)] {
            for accion in [
                AppAction::Save,
                AppAction::NewDocument,
                AppAction::FormatBold,
                AppAction::ToggleMode,
            ] {
                assert_eq!(surface_press(abierta, accion, true), SurfacePress::Run);
            }
        }
    }

    #[test]
    fn el_espacio_de_trabajo_abre_sus_archivos_si_ya_hay_carpeta() {
        assert_eq!(
            surface_for_action(AppAction::WorkspaceHub, true),
            Some(AppAction::WorkspaceFiles)
        );
        assert_eq!(
            surface_for_action(AppAction::WorkspaceHub, false),
            Some(AppAction::WorkspaceHub)
        );
        // Con carpeta, pulsar el hub con los archivos abiertos los cierra.
        assert_eq!(
            surface_press(
                Some(AppAction::WorkspaceFiles),
                AppAction::WorkspaceHub,
                true
            ),
            SurfacePress::Close
        );
        // Sin carpeta, el hub abierto (selector de carpeta) también se cierra.
        assert_eq!(
            surface_press(
                Some(AppAction::WorkspaceHub),
                AppAction::WorkspaceHub,
                false
            ),
            SurfacePress::Close
        );
    }

    #[test]
    fn un_boton_se_ve_activo_con_su_panel_abierto() {
        let abierta = Some(AppAction::DocumentOutline);
        assert!(toolbar_button_active(
            AppAction::DocumentOutline,
            DocumentMode::Reading,
            abierta,
            false
        ));
        assert!(!toolbar_button_active(
            AppAction::Backlinks,
            DocumentMode::Reading,
            abierta,
            false
        ));
        assert!(!toolbar_button_active(
            AppAction::DocumentOutline,
            DocumentMode::Reading,
            None,
            false
        ));
        // El modo de edición sigue marcando su botón.
        assert!(toolbar_button_active(
            AppAction::ToggleMode,
            DocumentMode::SourceEditing,
            None,
            false
        ));
    }

    #[test]
    fn open_surface_refleja_lo_que_esta_abierto_y_cerrar_lo_deja_en_nada() {
        let mut app = App::for_tests(NIGHT);
        app.load_for_test("# Uno\n\n## Dos\n", DocumentMode::Reading);
        assert_eq!(app.open_surface(), None);

        app.show_document_outline();
        assert_eq!(app.open_surface(), Some(AppAction::DocumentOutline));
        app.press_toolbar_action(AppAction::DocumentOutline);
        assert_eq!(app.open_surface(), None, "el mismo botón lo cierra");

        app.open_command_palette();
        assert_eq!(app.open_surface(), Some(AppAction::CommandPalette));
        // Otro botón de panel cambia directo, sin dejar la paleta abierta.
        app.press_toolbar_action(AppAction::DocumentOutline);
        assert_eq!(app.open_surface(), Some(AppAction::DocumentOutline));
        assert!(app.command_palette.is_none());

        app.open_document_search();
        app.close_open_surface();
        assert_eq!(app.open_surface(), None);
        assert!(app.search_query.is_none());
    }

    #[test]
    fn cerrar_un_panel_no_toca_el_documento() {
        let mut app = App::for_tests(NIGHT);
        app.load_for_test("texto original", DocumentMode::SourceEditing);
        let antes = app.document.source.to_string();
        app.open_command_palette();
        app.press_toolbar_action(AppAction::CommandPalette);
        assert_eq!(app.open_surface(), None);
        assert_eq!(app.document.source.to_string(), antes);
        assert!(!app.document.is_dirty());
    }
}
