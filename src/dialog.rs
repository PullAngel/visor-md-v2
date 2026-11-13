//! Diálogos propios de Visor MD.
//!
//! Reemplazan los cuadros nativos de Sí/No: cada botón nombra la acción que
//! ejecuta y Escape siempre elige la opción que conserva el trabajo. El modelo
//! no dibuja ni toca archivos; solo describe la pregunta y la respuesta
//! elegida. La aplicación decide qué hacer con cada `DialogChoice`.

/// Respuesta concreta de un diálogo. Cada variante corresponde a una única
/// acción de la aplicación; no existen respuestas genéricas como "Sí".
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DialogChoice {
    AcknowledgeRecovery,
    KeepRecovery,
    DisableRecovery,
    ReloadExternal,
    SaveCopy,
    KeepLocal,
    SaveAndCloseTab,
    DiscardAndCloseTab,
    CloseWindow,
    KeepEditing,
    LoadImage,
    SkipImage,
}

/// Peso visual de un botón. El primario es la acción recomendada; el
/// destructivo se distingue sin usar rojo saturado.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ButtonTone {
    Primary,
    Neutral,
    Destructive,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct DialogButton {
    pub(crate) label: &'static str,
    pub(crate) choice: DialogChoice,
    pub(crate) tone: ButtonTone,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Dialog {
    pub(crate) title: String,
    pub(crate) body: String,
    /// De izquierda a derecha. La acción primaria va al final, junto al borde
    /// derecho, como en la mayoría de los sistemas de escritorio.
    pub(crate) buttons: Vec<DialogButton>,
    pub(crate) focused: usize,
    /// Respuesta de Escape, clic fuera o cierre del diálogo.
    pub(crate) safe_choice: DialogChoice,
}

const fn button(label: &'static str, choice: DialogChoice, tone: ButtonTone) -> DialogButton {
    DialogButton {
        label,
        choice,
        tone,
    }
}

impl Dialog {
    fn new(
        title: impl Into<String>,
        body: impl Into<String>,
        buttons: Vec<DialogButton>,
        safe_choice: DialogChoice,
    ) -> Self {
        let focused = buttons
            .iter()
            .position(|button| button.tone == ButtonTone::Primary)
            .unwrap_or(0);
        Self {
            title: title.into(),
            body: body.into(),
            buttons,
            focused,
            safe_choice,
        }
    }

    pub(crate) fn recovery_privacy() -> Self {
        Self::new(
            "Recuperación local activada",
            "Visor MD guarda temporalmente los cambios sin guardar en tu perfil local para recuperarlos tras un cierre inesperado. Es texto sin cifrar, no reemplaza a Guardar y nunca se escribe dentro de tu carpeta de notas.",
            vec![button(
                "Entendido",
                DialogChoice::AcknowledgeRecovery,
                ButtonTone::Primary,
            )],
            DialogChoice::AcknowledgeRecovery,
        )
    }

    pub(crate) fn disable_recovery() -> Self {
        Self::new(
            "¿Desactivar la recuperación local?",
            "La recuperación conserva copias temporales sin cifrar de los documentos modificados. Sin ella, un cierre inesperado puede perder cambios. Al desactivarla se borran las copias de esta sesión.",
            vec![
                button(
                    "Desactivar",
                    DialogChoice::DisableRecovery,
                    ButtonTone::Destructive,
                ),
                button(
                    "Mantener activa",
                    DialogChoice::KeepRecovery,
                    ButtonTone::Primary,
                ),
            ],
            DialogChoice::KeepRecovery,
        )
    }

    pub(crate) fn external_change(dirty: bool) -> Self {
        let body = if dirty {
            "El archivo cambió fuera de Visor MD mientras tenías cambios sin guardar. Si recargás, tu versión queda en una recuperación local antes de reemplazarla."
        } else {
            "El archivo cambió fuera de Visor MD. Podés recargar la versión nueva o seguir con la que ves ahora."
        };
        Self::new(
            "El archivo cambió en el disco",
            body,
            vec![
                button(
                    "Guardar una copia",
                    DialogChoice::SaveCopy,
                    ButtonTone::Neutral,
                ),
                button(
                    "Mantener la mía",
                    DialogChoice::KeepLocal,
                    ButtonTone::Neutral,
                ),
                button(
                    "Recargar",
                    DialogChoice::ReloadExternal,
                    ButtonTone::Primary,
                ),
            ],
            DialogChoice::KeepLocal,
        )
    }

    pub(crate) fn external_unavailable() -> Self {
        Self::new(
            "El archivo ya no está disponible",
            "Se borró, dejó de ser un archivo normal o ya no se puede leer. Tu documento sigue abierto e intacto aquí.",
            vec![
                button(
                    "Seguir editando",
                    DialogChoice::KeepLocal,
                    ButtonTone::Neutral,
                ),
                button(
                    "Guardar una copia",
                    DialogChoice::SaveCopy,
                    ButtonTone::Primary,
                ),
            ],
            DialogChoice::KeepLocal,
        )
    }

    pub(crate) fn save_conflict() -> Self {
        Self::new(
            "No se guardó: el archivo cambió",
            "Otro programa modificó el archivo después de que lo abriste, así que no se sobrescribió. Si recargás, tu versión queda antes en una recuperación local.",
            vec![
                button(
                    "Recargar la externa",
                    DialogChoice::ReloadExternal,
                    ButtonTone::Destructive,
                ),
                button(
                    "Mantener la mía",
                    DialogChoice::KeepLocal,
                    ButtonTone::Neutral,
                ),
                button(
                    "Guardar una copia",
                    DialogChoice::SaveCopy,
                    ButtonTone::Primary,
                ),
            ],
            DialogChoice::KeepLocal,
        )
    }

    pub(crate) fn close_tab(name: &str, recovery_preserved: bool) -> Self {
        let body = if recovery_preserved {
            "Si cerrás sin guardar, los cambios quedan en una recuperación local que podés restaurar después."
        } else {
            "La recuperación local está desactivada: si cerrás sin guardar, estos cambios se pierden."
        };
        Self::new(
            format!("¿Guardar los cambios de «{name}»?"),
            body,
            vec![
                button(
                    "Cerrar sin guardar",
                    DialogChoice::DiscardAndCloseTab,
                    ButtonTone::Destructive,
                ),
                button("Cancelar", DialogChoice::KeepEditing, ButtonTone::Neutral),
                button(
                    "Guardar",
                    DialogChoice::SaveAndCloseTab,
                    ButtonTone::Primary,
                ),
            ],
            DialogChoice::KeepEditing,
        )
    }

    pub(crate) fn close_window(dirty_count: usize, recovery_preserved: bool) -> Self {
        let documents = if dirty_count == 1 {
            "Hay 1 documento con cambios sin guardar.".to_string()
        } else {
            format!("Hay {dirty_count} documentos con cambios sin guardar.")
        };
        let consequence = if recovery_preserved {
            " Si cerrás, quedan en recuperaciones locales que podés restaurar al volver."
        } else {
            " La recuperación local está desactivada: si cerrás, esos cambios se pierden."
        };
        Self::new(
            "¿Cerrar Visor MD?",
            documents + consequence,
            vec![
                button(
                    "Cerrar de todos modos",
                    DialogChoice::CloseWindow,
                    ButtonTone::Destructive,
                ),
                button(
                    "Seguir editando",
                    DialogChoice::KeepEditing,
                    ButtonTone::Primary,
                ),
            ],
            DialogChoice::KeepEditing,
        )
    }

    pub(crate) fn local_image() -> Self {
        Self::new(
            "¿Mostrar esta imagen local?",
            "Se leerá una sola imagen PNG dentro de la carpeta de trabajo, con límites de 8 MiB y 16 millones de píxeles. El permiso no se recuerda.",
            vec![
                button("No cargar", DialogChoice::SkipImage, ButtonTone::Neutral),
                button(
                    "Mostrar imagen",
                    DialogChoice::LoadImage,
                    ButtonTone::Primary,
                ),
            ],
            DialogChoice::SkipImage,
        )
    }

    /// Tab y las flechas recorren los botones en ciclo, sin salir del diálogo.
    pub(crate) fn move_focus(&mut self, backwards: bool) {
        let count = self.buttons.len();
        if count == 0 {
            return;
        }
        self.focused = if backwards {
            (self.focused + count - 1) % count
        } else {
            (self.focused + 1) % count
        };
    }

    pub(crate) fn focused_choice(&self) -> DialogChoice {
        self.buttons
            .get(self.focused)
            .map_or(self.safe_choice, |button| button.choice)
    }
}

/// Geometría compartida por el dibujo y la detección de clics.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct DialogRect {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
}

impl DialogRect {
    pub(crate) fn contains(self, x: f32, y: f32) -> bool {
        (self.x..self.x + self.width).contains(&x) && (self.y..self.y + self.height).contains(&y)
    }
}

pub(crate) const DIALOG_MAX_WIDTH: f32 = 440.0;
pub(crate) const DIALOG_PADDING: f32 = 24.0;
pub(crate) const DIALOG_BUTTON_HEIGHT: f32 = 34.0;
pub(crate) const DIALOG_BUTTON_GAP: f32 = 8.0;
pub(crate) const DIALOG_BUTTON_PADDING: f32 = 16.0;
const DIALOG_TITLE_GAP: f32 = 10.0;
const DIALOG_BODY_GAP: f32 = 22.0;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DialogLayout {
    pub(crate) panel: DialogRect,
    pub(crate) title_y: f32,
    pub(crate) body_y: f32,
    pub(crate) buttons: Vec<DialogRect>,
}

/// Ancho del panel para una ventana dada. Deja siempre un margen visible.
pub(crate) fn dialog_width(window_width: f32) -> f32 {
    DIALOG_MAX_WIDTH.min((window_width - 32.0).max(200.0))
}

/// Ubica el panel centrado y los botones alineados a la derecha. Si no caben
/// en una fila, se apilan a todo el ancho con la acción primaria abajo.
pub(crate) fn dialog_layout(
    window_width: f32,
    window_height: f32,
    title_height: f32,
    body_height: f32,
    label_widths: &[f32],
) -> DialogLayout {
    let width = dialog_width(window_width);
    let inner = width - DIALOG_PADDING * 2.0;
    let button_widths = label_widths
        .iter()
        .map(|label| (label + DIALOG_BUTTON_PADDING * 2.0).max(88.0))
        .collect::<Vec<_>>();
    let row_width = button_widths.iter().sum::<f32>()
        + DIALOG_BUTTON_GAP * button_widths.len().saturating_sub(1) as f32;
    let stacked = row_width > inner;
    let buttons_height = if stacked {
        button_widths.len() as f32 * (DIALOG_BUTTON_HEIGHT + DIALOG_BUTTON_GAP) - DIALOG_BUTTON_GAP
    } else {
        DIALOG_BUTTON_HEIGHT
    };
    let height = DIALOG_PADDING
        + title_height
        + DIALOG_TITLE_GAP
        + body_height
        + DIALOG_BODY_GAP
        + buttons_height
        + DIALOG_PADDING;
    let x = ((window_width - width) * 0.5).round();
    let y = ((window_height - height) * 0.45).round().max(8.0);
    let title_y = y + DIALOG_PADDING;
    let body_y = title_y + title_height + DIALOG_TITLE_GAP;
    let buttons_top = body_y + body_height + DIALOG_BODY_GAP;
    let buttons = if stacked {
        button_widths
            .iter()
            .enumerate()
            .map(|(index, _)| DialogRect {
                x: x + DIALOG_PADDING,
                y: buttons_top + index as f32 * (DIALOG_BUTTON_HEIGHT + DIALOG_BUTTON_GAP),
                width: inner,
                height: DIALOG_BUTTON_HEIGHT,
            })
            .collect()
    } else {
        let mut cursor = x + width - DIALOG_PADDING - row_width;
        button_widths
            .iter()
            .map(|button_width| {
                let rect = DialogRect {
                    x: cursor,
                    y: buttons_top,
                    width: *button_width,
                    height: DIALOG_BUTTON_HEIGHT,
                };
                cursor += button_width + DIALOG_BUTTON_GAP;
                rect
            })
            .collect()
    };
    DialogLayout {
        panel: DialogRect {
            x,
            y,
            width,
            height,
        },
        title_y,
        body_y,
        buttons,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cada_dialogo_nombra_acciones_y_escape_conserva_el_trabajo() {
        let dialogs = [
            Dialog::recovery_privacy(),
            Dialog::disable_recovery(),
            Dialog::external_change(true),
            Dialog::external_unavailable(),
            Dialog::save_conflict(),
            Dialog::close_tab("nota.md", true),
            Dialog::close_window(2, false),
            Dialog::local_image(),
        ];
        for dialog in dialogs {
            assert!(!dialog.buttons.is_empty());
            for button in &dialog.buttons {
                assert!(!matches!(button.label, "Sí" | "No" | "Aceptar"));
            }
            assert!(!matches!(
                dialog.safe_choice,
                DialogChoice::DiscardAndCloseTab
                    | DialogChoice::CloseWindow
                    | DialogChoice::DisableRecovery
                    | DialogChoice::ReloadExternal
            ));
            assert_eq!(dialog.buttons[dialog.focused].tone, ButtonTone::Primary);
            assert_ne!(dialog.focused_choice(), DialogChoice::DiscardAndCloseTab);
        }
    }

    #[test]
    fn el_foco_recorre_los_botones_en_ciclo() {
        let mut dialog = Dialog::close_tab("nota.md", true);
        assert_eq!(dialog.focused_choice(), DialogChoice::SaveAndCloseTab);
        dialog.move_focus(false);
        assert_eq!(dialog.focused_choice(), DialogChoice::DiscardAndCloseTab);
        dialog.move_focus(true);
        dialog.move_focus(true);
        assert_eq!(dialog.focused_choice(), DialogChoice::KeepEditing);
    }

    #[test]
    fn los_botones_caben_dentro_del_panel_o_se_apilan() {
        let wide = dialog_layout(1200.0, 800.0, 20.0, 60.0, &[90.0, 60.0, 50.0]);
        for button in &wide.buttons {
            assert!(button.x >= wide.panel.x + DIALOG_PADDING - 0.5);
            assert!(
                button.x + button.width <= wide.panel.x + wide.panel.width - DIALOG_PADDING + 0.5
            );
            assert!(button.y + button.height <= wide.panel.y + wide.panel.height);
        }
        assert!(wide.buttons.windows(2).all(|pair| pair[0].y == pair[1].y));

        let narrow = dialog_layout(320.0, 600.0, 20.0, 80.0, &[150.0, 120.0, 90.0]);
        assert!(narrow.buttons.windows(2).all(|pair| pair[0].y < pair[1].y));
        assert!(narrow.panel.x >= 0.0 && narrow.panel.x + narrow.panel.width <= 320.0);
        let last = narrow.buttons.last().expect("hay botones");
        assert!(last.y + last.height <= narrow.panel.y + narrow.panel.height);
    }
}
