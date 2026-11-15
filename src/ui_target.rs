//! Objetivos interactivos de la interfaz. Una sola función decide qué hay bajo
//! el puntero, usando las mismas geometrías que el dibujo y el clic. Así el
//! cursor de mano, el hover y el clic no pueden desincronizarse.
//!
//! Hoy cubre barras, selector de modo, pestañas, controles de ventana, Copiar,
//! tareas, plegado, menú contextual y botones de diálogo. Las filas de listas y
//! las cabeceras de panel se suman con sus fases (3 y 5 del plan).

use super::*;

/// Lo que se puede tocar. Es `Copy` y no contiene referencias: sirve de clave
/// para el estado de hover y de pulsación.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum UiTarget {
    /// Botón de la barra principal o de la contextual.
    Toolbar(AppAction),
    ModeSegment(ModeTarget),
    Tab(usize),
    TabClose(usize),
    NewTab,
    WindowControl(WindowControl),
    CodeCopy {
        block: usize,
        preview: bool,
    },
    TaskCheckbox(usize),
    SectionDisclosure(usize),
    DialogButton(usize),
    MenuRow(usize),
}

/// Objetivo del cromo de la ventana (controles, barras y pestañas) bajo el
/// punto, en el orden en que el clic los atiende. No depende del estado de la
/// app más que del modo y la cantidad de pestañas.
pub(crate) fn chrome_target_at(
    x: f32,
    y: f32,
    window_width: f32,
    window_height: f32,
    mode: DocumentMode,
    tab_count: usize,
) -> Option<UiTarget> {
    if let Some(control) = window_control_at(x, y, window_width) {
        return Some(UiTarget::WindowControl(control));
    }
    if let Some(segment) = mode_target_at(x, y) {
        return Some(UiTarget::ModeSegment(segment));
    }
    if let Some(action) = toolbar_action_at(x, y, window_width, mode) {
        return Some(UiTarget::Toolbar(action));
    }
    if new_tab_button_at(x, y, window_width, tab_count) {
        return Some(UiTarget::NewTab);
    }
    if let Some(index) = tab_close_index_at(x, y, window_width, window_height, tab_count) {
        return Some(UiTarget::TabClose(index));
    }
    tab_index_at(x, y, window_width, window_height, tab_count).map(UiTarget::Tab)
}

impl App {
    /// Tamaño del cuadro: el de la ventana o, sin ella, el del último lienzo.
    pub(crate) fn view_size(&self) -> Option<(f32, f32)> {
        if let Some(window) = &self.window {
            let size = window.inner_size();
            return Some((size.width as f32, size.height as f32));
        }
        self.pixmap
            .as_ref()
            .map(|pixmap| (pixmap.width() as f32, pixmap.height() as f32))
    }

    /// Qué hay bajo el punto, en orden de capas: un diálogo captura todo, luego
    /// el menú contextual y por último el cromo y el documento.
    pub(crate) fn ui_target_at(&self, x: f32, y: f32) -> Option<UiTarget> {
        if self.dialog.is_some() {
            return self
                .dialog_buttons
                .iter()
                .position(|rect| rect.contains(x, y))
                .map(UiTarget::DialogButton);
        }
        if let Some(menu) = &self.context_menu {
            return context_action_at(menu, (x, y))
                .and_then(|action| menu.actions.iter().position(|item| *item == action))
                .map(UiTarget::MenuRow);
        }
        let (width, height) = self.view_size()?;
        if let Some(target) = chrome_target_at(
            x,
            y,
            width,
            height,
            self.document.mode,
            self.tab_order.len(),
        ) {
            return Some(target);
        }
        if let Some((block, preview)) = self.code_copy_at(x, y) {
            return Some(UiTarget::CodeCopy { block, preview });
        }
        if let Some(block) = self.task_at(x, y) {
            return Some(UiTarget::TaskCheckbox(block));
        }
        self.heading_disclosure_at(x, y)
            .map(UiTarget::SectionDisclosure)
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    const ANCHOS: [f32; 3] = [640.0, 1100.0, 1366.0];
    const ALTO: f32 = 720.0;

    fn objetivo(
        x: f32,
        y: f32,
        ancho: f32,
        modo: DocumentMode,
        pestanas: usize,
    ) -> Option<UiTarget> {
        chrome_target_at(x, y, ancho, ALTO, modo, pestanas)
    }

    #[test]
    fn el_centro_de_cada_control_de_ventana_devuelve_su_objetivo() {
        // Los controles propios solo existen con el cromo propio (Windows).
        if !custom_window_chrome_enabled() {
            return;
        }
        let y = WINDOW_CHROME_HEIGHT * 0.5;
        let controles = [
            WindowControl::PinOnTop,
            WindowControl::Minimize,
            WindowControl::ToggleMaximize,
            WindowControl::Close,
        ];
        for ancho in ANCHOS {
            for (indice, control) in controles.iter().enumerate() {
                let x = ancho - WINDOW_CONTROL_WIDTH * (WINDOW_CONTROL_COUNT - indice as f32 - 0.5);
                assert_eq!(
                    objetivo(x, y, ancho, DocumentMode::Reading, 1),
                    Some(UiTarget::WindowControl(*control)),
                    "{control:?} a {ancho} px"
                );
            }
        }
    }

    #[test]
    fn el_centro_de_cada_segmento_de_modo_devuelve_su_objetivo() {
        let y = TOOLBAR_Y + MODE_SWITCH_HEIGHT * 0.5;
        for ancho in ANCHOS {
            for (indice, segmento) in [
                ModeTarget::Reading,
                ModeTarget::Editing,
                ModeTarget::Compare,
            ]
            .into_iter()
            .enumerate()
            {
                let x = MODE_SWITCH_X + MODE_SWITCH_WIDTH * (indice as f32 + 0.5) / 3.0;
                assert_eq!(
                    objetivo(x, y, ancho, DocumentMode::Reading, 1),
                    Some(UiTarget::ModeSegment(segmento)),
                    "{segmento:?} a {ancho} px"
                );
            }
        }
    }

    #[test]
    fn el_centro_de_cada_boton_de_la_barra_principal_devuelve_su_accion() {
        let y = TOOLBAR_Y + TOOLBAR_HEIGHT * 0.5;
        for ancho in ANCHOS {
            for (indice, accion) in PRIMARY_TOOLBAR_ACTIONS.iter().enumerate() {
                let x = TOOLBAR_X
                    + indice as f32 * TOOLBAR_ITEM_WIDTH
                    + (TOOLBAR_ITEM_WIDTH - 4.0) * 0.5;
                assert_eq!(
                    objetivo(x, y, ancho, DocumentMode::Reading, 1),
                    Some(UiTarget::Toolbar(*accion)),
                    "{accion:?} a {ancho} px"
                );
            }
        }
    }

    #[test]
    fn las_pestanas_su_cierre_y_el_boton_nuevo_se_distinguen() {
        let y = TAB_STRIP_Y + TAB_STRIP_HEIGHT * 0.5;
        for ancho in ANCHOS {
            for cantidad in [1, 3] {
                let ancho_pestana = tab_width(ancho, cantidad);
                for indice in 0..cantidad {
                    let inicio = TAB_STRIP_X + indice as f32 * ancho_pestana;
                    assert_eq!(
                        objetivo(inicio + 20.0, y, ancho, DocumentMode::Reading, cantidad),
                        Some(UiTarget::Tab(indice)),
                        "pestaña {indice} de {cantidad} a {ancho} px"
                    );
                    assert_eq!(
                        objetivo(
                            inicio + ancho_pestana - 12.0,
                            y,
                            ancho,
                            DocumentMode::Reading,
                            cantidad
                        ),
                        Some(UiTarget::TabClose(indice)),
                        "cierre {indice} de {cantidad} a {ancho} px"
                    );
                }
                let x = new_tab_button_x(ancho, cantidad) + NEW_TAB_BUTTON_WIDTH * 0.5;
                assert_eq!(
                    objetivo(x, y, ancho, DocumentMode::Reading, cantidad),
                    Some(UiTarget::NewTab),
                    "nueva pestaña con {cantidad} a {ancho} px"
                );
            }
        }
    }

    #[test]
    fn el_documento_y_el_vacio_no_son_objetivos_del_cromo() {
        for ancho in ANCHOS {
            assert_eq!(
                objetivo(
                    300.0,
                    DOCUMENT_VIEWPORT_TOP + 60.0,
                    ancho,
                    DocumentMode::Reading,
                    1
                ),
                None
            );
            assert_eq!(objetivo(-5.0, 20.0, ancho, DocumentMode::Reading, 1), None);
        }
    }

    #[test]
    fn la_app_sin_ventana_resuelve_objetivos_con_el_tamano_del_lienzo() {
        let mut app = App::for_tests(NIGHT);
        app.load_for_test("# Hola\n\n```\ncódigo\n```\n", DocumentMode::Reading);
        app.draw_for_test(1100, 720);
        let x = TOOLBAR_X + (TOOLBAR_ITEM_WIDTH - 4.0) * 0.5;
        let y = TOOLBAR_Y + TOOLBAR_HEIGHT * 0.5;
        assert_eq!(
            app.ui_target_at(x, y),
            Some(UiTarget::Toolbar(PRIMARY_TOOLBAR_ACTIONS[0]))
        );
        // Con un diálogo abierto, solo sus botones son objetivos.
        let id = app.document.id;
        app.open_dialog(Dialog::close_tab("apuntes.md", true), Some(id));
        app.draw_for_test(1100, 720);
        assert_eq!(app.ui_target_at(x, y), None);
        let boton = app.dialog_buttons[0];
        assert_eq!(
            app.ui_target_at(boton.x + boton.width * 0.5, boton.y + boton.height * 0.5),
            Some(UiTarget::DialogButton(0))
        );
    }
}
