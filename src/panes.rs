//! Árbol de paneles de documentos y su geometría.
//!
//! Cada hoja referencia un documento de la sesión, nunca una copia de su
//! buffer. Este módulo no conoce ventanas, archivos ni dibujo: solo decide qué
//! rectángulo ocupa cada hoja y qué control hay bajo un punto. Así el mismo
//! modelo sirve para escritorio y, a futuro, para la versión móvil.

use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};

/// Identidad efímera de una vista. No coincide con la identidad del documento:
/// cada panel muestra un documento distinto de la misma sesión, sin crear
/// buffers, historiales ni archivos de recuperación adicionales.
static NEXT_DOCUMENT_PANE_ID: AtomicU64 = AtomicU64::new(1);

use crate::{
    MAX_DOCUMENT_PANES, ModeTarget, PANE_HEADER_CLOSE_WIDTH, PANE_HEADER_HEIGHT,
    PANE_HEADER_MODE_MIN_WIDTH, PANE_HEADER_MODE_SLOT,
};

/// Ancho del borde arrastrable entre dos paneles.
pub(crate) const DIVIDER_THICKNESS: f32 = 6.0;
/// Ningún panel baja de este tamaño útil al arrastrar un divisor.
pub(crate) const MIN_PANE_EXTENT: f32 = 240.0;
/// Cuánto mueve un paso de teclado.
pub(crate) const DIVIDER_KEY_STEP: f32 = 0.05;

/// Un divisor entre los dos hijos de una división. `index` es el orden en que
/// el árbol visita las divisiones (primero la raíz): identifica al divisor
/// mientras no cambie la estructura.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PaneDivider {
    pub(crate) index: usize,
    pub(crate) axis: DocumentPaneAxis,
    /// Todo el espacio que reparte esta división.
    pub(crate) region: PaneGeometry,
    /// La franja arrastrable, centrada en el borde.
    pub(crate) rect: PaneGeometry,
    pub(crate) fraction: f32,
}

/// Dirección de una división entre documentos. Es distinta de la comparación
/// fuente/vista: cada hoja del árbol referencia una vista de un documento de
/// sesión, nunca una copia de su buffer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) enum DocumentPaneAxis {
    Vertical,
    Horizontal,
}

impl DocumentPaneAxis {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Vertical => "a la derecha",
            Self::Horizontal => "abajo",
        }
    }
}

/// Modelo de disposición de documentos para vistas divididas. Cada hoja tiene
/// una identidad de vista estable y referencia un documento de sesión. No posee
/// buffers ni recuperación: esos datos continúan perteneciendo al documento.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum DocumentPaneTree {
    Leaf {
        pane_id: u64,
        document_id: u64,
        /// El desplazamiento pertenece a la vista, no a la fuente. Cuando el
        /// mosaico sea visible, cada archivo conservará su propia posición.
        scroll: f32,
    },
    Split {
        axis: DocumentPaneAxis,
        fraction: f32,
        first: Box<DocumentPaneTree>,
        second: Box<DocumentPaneTree>,
    },
}

impl DocumentPaneTree {
    pub(crate) fn single(document_id: u64) -> Self {
        Self::new_leaf(document_id)
    }

    pub(crate) fn new_leaf(document_id: u64) -> Self {
        Self::Leaf {
            pane_id: NEXT_DOCUMENT_PANE_ID.fetch_add(1, Ordering::Relaxed),
            document_id,
            scroll: 0.0,
        }
    }

    pub(crate) fn first_pane_id(&self) -> u64 {
        match self {
            Self::Leaf { pane_id, .. } => *pane_id,
            Self::Split { first, .. } => first.first_pane_id(),
        }
    }

    pub(crate) fn pane_for_document(&self, document_id: u64) -> Option<u64> {
        match self {
            Self::Leaf {
                pane_id,
                document_id: current,
                ..
            } => (*current == document_id).then_some(*pane_id),
            Self::Split { first, second, .. } => first
                .pane_for_document(document_id)
                .or_else(|| second.pane_for_document(document_id)),
        }
    }

    pub(crate) fn pane_count(&self) -> usize {
        match self {
            Self::Leaf { .. } => 1,
            Self::Split { first, second, .. } => first.pane_count() + second.pane_count(),
        }
    }

    pub(crate) fn contains_pane(&self, pane_id: u64) -> bool {
        match self {
            Self::Leaf {
                pane_id: current, ..
            } => *current == pane_id,
            Self::Split { first, second, .. } => {
                first.contains_pane(pane_id) || second.contains_pane(pane_id)
            }
        }
    }

    pub(crate) fn document_for_pane(&self, pane_id: u64) -> Option<u64> {
        match self {
            Self::Leaf {
                pane_id: current,
                document_id,
                ..
            } => (*current == pane_id).then_some(*document_id),
            Self::Split { first, second, .. } => first
                .document_for_pane(pane_id)
                .or_else(|| second.document_for_pane(pane_id)),
        }
    }

    pub(crate) fn pane_scroll(&self, pane_id: u64) -> Option<f32> {
        match self {
            Self::Leaf {
                pane_id: current,
                scroll,
                ..
            } => (*current == pane_id).then_some(*scroll),
            Self::Split { first, second, .. } => first
                .pane_scroll(pane_id)
                .or_else(|| second.pane_scroll(pane_id)),
        }
    }

    pub(crate) fn set_pane_scroll(&mut self, pane_id: u64, scroll: f32) -> bool {
        let scroll = scroll.max(0.0);
        match self {
            Self::Leaf {
                pane_id: current,
                scroll: current_scroll,
                ..
            } if *current == pane_id => {
                *current_scroll = scroll;
                true
            }
            Self::Leaf { .. } => false,
            Self::Split { first, second, .. } => {
                first.set_pane_scroll(pane_id, scroll) || second.set_pane_scroll(pane_id, scroll)
            }
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn document_ids(&self, output: &mut Vec<u64>) {
        match self {
            Self::Leaf { document_id, .. } => output.push(*document_id),
            Self::Split { first, second, .. } => {
                first.document_ids(output);
                second.document_ids(output);
            }
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn references_only(&self, document_ids: &HashSet<u64>) -> bool {
        match self {
            Self::Leaf { document_id, .. } => document_ids.contains(document_id),
            Self::Split { first, second, .. } => {
                first.references_only(document_ids) && second.references_only(document_ids)
            }
        }
    }

    /// La hoja recibe la posición que el documento guardó al dejar de verse.
    /// Reiniciarla a cero hacía que cambiar de pestaña volviera al inicio.
    pub(crate) fn replace_pane_document(
        &mut self,
        pane_id: u64,
        document_id: u64,
        document_scroll: f32,
    ) -> bool {
        if self.pane_for_document(document_id).is_some() {
            return false;
        }
        match self {
            Self::Leaf {
                pane_id: current_pane,
                document_id: current_document,
                scroll,
            } if *current_pane == pane_id => {
                *current_document = document_id;
                *scroll = document_scroll.max(0.0);
                true
            }
            Self::Leaf { .. } => false,
            Self::Split { first, second, .. } => {
                first.replace_pane_document(pane_id, document_id, document_scroll)
                    || second.replace_pane_document(pane_id, document_id, document_scroll)
            }
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn split_pane(
        &mut self,
        pane_id: u64,
        adjacent_id: u64,
        axis: DocumentPaneAxis,
    ) -> Option<u64> {
        if self.pane_count() >= MAX_DOCUMENT_PANES || self.pane_for_document(adjacent_id).is_some()
        {
            return None;
        }
        match self {
            Self::Leaf {
                pane_id: current_pane,
                document_id: current_document,
                scroll,
            } if *current_pane == pane_id => {
                let adjacent = Self::new_leaf(adjacent_id);
                let adjacent_pane_id = adjacent.first_pane_id();
                *self = Self::Split {
                    axis,
                    fraction: 0.5,
                    first: Box::new(Self::Leaf {
                        pane_id,
                        document_id: *current_document,
                        scroll: *scroll,
                    }),
                    second: Box::new(adjacent),
                };
                Some(adjacent_pane_id)
            }
            Self::Leaf { .. } => None,
            Self::Split { first, second, .. } => first
                .split_pane(pane_id, adjacent_id, axis)
                .or_else(|| second.split_pane(pane_id, adjacent_id, axis)),
        }
    }

    pub(crate) fn without_document(&self, document_id: u64) -> Option<Self> {
        match self {
            Self::Leaf {
                document_id: current,
                ..
            } => (*current != document_id).then(|| self.clone()),
            Self::Split {
                axis,
                fraction,
                first,
                second,
            } => match (
                first.without_document(document_id),
                second.without_document(document_id),
            ) {
                (Some(first), Some(second)) => Some(Self::Split {
                    axis: *axis,
                    fraction: normalized_pane_fraction(*fraction),
                    first: Box::new(first),
                    second: Box::new(second),
                }),
                (Some(remaining), None) | (None, Some(remaining)) => Some(remaining),
                (None, None) => None,
            },
        }
    }

    /// Cierra solo una hoja visual. El documento queda abierto en su pestaña,
    /// con fuente, historial y recuperación intactos, y el árbol colapsa la
    /// rama que ya no tiene dos hijos.
    pub(crate) fn without_pane(&self, pane_id: u64) -> Option<Self> {
        match self {
            Self::Leaf {
                pane_id: current, ..
            } => (*current != pane_id).then(|| self.clone()),
            Self::Split {
                axis,
                fraction,
                first,
                second,
            } => match (first.without_pane(pane_id), second.without_pane(pane_id)) {
                (Some(first), Some(second)) => Some(Self::Split {
                    axis: *axis,
                    fraction: normalized_pane_fraction(*fraction),
                    first: Box::new(first),
                    second: Box::new(second),
                }),
                (Some(remaining), None) | (None, Some(remaining)) => Some(remaining),
                (None, None) => None,
            },
        }
    }

    /// Los divisores del árbol dentro de `available`, en el orden de visita.
    pub(crate) fn dividers(&self, available: PaneGeometry) -> Vec<PaneDivider> {
        let mut output = Vec::new();
        self.collect_dividers(available, &mut 0, &mut output);
        output
    }

    fn collect_dividers(
        &self,
        available: PaneGeometry,
        next: &mut usize,
        output: &mut Vec<PaneDivider>,
    ) {
        let Self::Split {
            axis,
            fraction,
            first,
            second,
        } = self
        else {
            return;
        };
        let index = *next;
        *next += 1;
        let fraction = normalized_pane_fraction(*fraction);
        let (first_geometry, second_geometry, rect) = match axis {
            DocumentPaneAxis::Vertical => {
                let first_width = split_pane_extent(available.width, fraction);
                (
                    PaneGeometry {
                        width: first_width,
                        ..available
                    },
                    PaneGeometry {
                        x: available.x + first_width,
                        width: available.width - first_width,
                        ..available
                    },
                    PaneGeometry {
                        x: available.x + first_width - DIVIDER_THICKNESS * 0.5,
                        width: DIVIDER_THICKNESS,
                        ..available
                    },
                )
            }
            DocumentPaneAxis::Horizontal => {
                let first_height = split_pane_extent(available.height, fraction);
                (
                    PaneGeometry {
                        height: first_height,
                        ..available
                    },
                    PaneGeometry {
                        y: available.y + first_height,
                        height: available.height - first_height,
                        ..available
                    },
                    PaneGeometry {
                        y: available.y + first_height - DIVIDER_THICKNESS * 0.5,
                        height: DIVIDER_THICKNESS,
                        ..available
                    },
                )
            }
        };
        output.push(PaneDivider {
            index,
            axis: *axis,
            region: available,
            rect,
            fraction,
        });
        first.collect_dividers(first_geometry, next, output);
        second.collect_dividers(second_geometry, next, output);
    }

    /// Fija la proporción de la división número `index` (orden de visita).
    pub(crate) fn set_split_fraction(&mut self, index: usize, new_fraction: f32) -> bool {
        self.set_split_fraction_from(&mut 0, index, new_fraction)
    }

    fn set_split_fraction_from(
        &mut self,
        next: &mut usize,
        index: usize,
        new_fraction: f32,
    ) -> bool {
        let Self::Split {
            fraction,
            first,
            second,
            ..
        } = self
        else {
            return false;
        };
        if *next == index {
            *fraction = normalized_pane_fraction(new_fraction);
            return true;
        }
        *next += 1;
        first.set_split_fraction_from(next, index, new_fraction)
            || second.set_split_fraction_from(next, index, new_fraction)
    }

    /// La división más cercana a la hoja `pane_id` que reparte en el sentido
    /// `axis`: su índice y su proporción actual. Sirve al teclado: «izquierda» y
    /// «derecha» mueven un divisor vertical, «arriba» y «abajo» uno horizontal.
    pub(crate) fn nearest_split_of(
        &self,
        pane_id: u64,
        axis: DocumentPaneAxis,
    ) -> Option<(usize, f32)> {
        self.nearest_split_from(&mut 0, pane_id, axis)
    }

    fn nearest_split_from(
        &self,
        next: &mut usize,
        pane_id: u64,
        wanted: DocumentPaneAxis,
    ) -> Option<(usize, f32)> {
        let Self::Split {
            axis,
            fraction,
            first,
            second,
        } = self
        else {
            return None;
        };
        let index = *next;
        *next += 1;
        let in_first = first.contains_pane(pane_id);
        let in_second = second.contains_pane(pane_id);
        if in_first && let Some(found) = first.nearest_split_from(next, pane_id, wanted) {
            return Some(found);
        }
        if in_first {
            *next += second.split_count();
        }
        if in_second && let Some(found) = second.nearest_split_from(next, pane_id, wanted) {
            return Some(found);
        }
        ((in_first || in_second) && *axis == wanted)
            .then_some((index, normalized_pane_fraction(*fraction)))
    }

    fn split_count(&self) -> usize {
        match self {
            Self::Leaf { .. } => 0,
            Self::Split { first, second, .. } => 1 + first.split_count() + second.split_count(),
        }
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn layout(&self, available: PaneGeometry, output: &mut Vec<DocumentPaneLayout>) {
        match self {
            Self::Leaf {
                pane_id,
                document_id,
                ..
            } => output.push(DocumentPaneLayout {
                pane_id: *pane_id,
                document_id: *document_id,
                geometry: available,
            }),
            Self::Split {
                axis,
                fraction,
                first,
                second,
            } => {
                let fraction = normalized_pane_fraction(*fraction);
                let (first_geometry, second_geometry) = match axis {
                    DocumentPaneAxis::Vertical => {
                        let first_width = split_pane_extent(available.width, fraction);
                        (
                            PaneGeometry {
                                width: first_width,
                                ..available
                            },
                            PaneGeometry {
                                x: available.x + first_width,
                                width: available.width - first_width,
                                ..available
                            },
                        )
                    }
                    DocumentPaneAxis::Horizontal => {
                        let first_height = split_pane_extent(available.height, fraction);
                        (
                            PaneGeometry {
                                height: first_height,
                                ..available
                            },
                            PaneGeometry {
                                y: available.y + first_height,
                                height: available.height - first_height,
                                ..available
                            },
                        )
                    }
                };
                first.layout(first_geometry, output);
                second.layout(second_geometry, output);
            }
        }
    }
}

/// Al cerrar una pestaña que estaba visible, el siguiente documento activo se
/// toma de una hoja superviviente. Esto conserva la coherencia entre árbol,
/// foco y caché aunque el orden de las pestañas haya sido reordenado aparte.
pub(crate) fn remaining_document_panes_after_close(
    panes: &DocumentPaneTree,
    closing_document: u64,
) -> Option<(DocumentPaneTree, u64)> {
    let remaining = panes.without_document(closing_document)?;
    let successor = remaining.document_for_pane(remaining.first_pane_id())?;
    Some((remaining, successor))
}

/// Reparte una dimensión incluso cuando una llamada de plataforma entrega un
/// tamaño transitorio de cero o uno durante un resize. No se crea geometría
/// negativa ni se usa `clamp` con límites invertidos.
pub(crate) fn split_pane_extent(total: f32, fraction: f32) -> f32 {
    let total = total.max(0.0);
    let minimum = if total >= 2.0 { 1.0 } else { 0.0 };
    (total * normalized_pane_fraction(fraction))
        .round()
        .clamp(minimum, total - minimum)
}

pub(crate) fn normalized_pane_fraction(fraction: f32) -> f32 {
    if fraction.is_finite() {
        fraction.clamp(0.15, 0.85)
    } else {
        0.5
    }
}

/// La proporción que corresponde a un puntero en `(x, y)` (coordenadas del área
/// documental) al arrastrar `divider`. Respeta el mínimo útil por panel y los
/// límites de `normalized_pane_fraction`; si la región es demasiado chica para
/// ambos mínimos, vuelve al centro.
pub(crate) fn divider_fraction_for_pointer(divider: PaneDivider, x: f32, y: f32) -> f32 {
    let (offset, extent) = match divider.axis {
        DocumentPaneAxis::Vertical => (x - divider.region.x, divider.region.width),
        DocumentPaneAxis::Horizontal => (y - divider.region.y, divider.region.height),
    };
    if extent <= 0.0 {
        return 0.5;
    }
    let minimum = (MIN_PANE_EXTENT / extent).max(0.15);
    let maximum = (1.0 - MIN_PANE_EXTENT / extent).min(0.85);
    if minimum > maximum {
        return 0.5;
    }
    (offset / extent).clamp(minimum, maximum)
}

/// El divisor bajo un punto, si lo hay.
pub(crate) fn divider_at(dividers: &[PaneDivider], x: f32, y: f32) -> Option<PaneDivider> {
    dividers
        .iter()
        .copied()
        .find(|divider| divider.rect.contains(x, y))
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PaneGeometry {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct DocumentPaneLayout {
    pub(crate) pane_id: u64,
    pub(crate) document_id: u64,
    pub(crate) geometry: PaneGeometry,
}

impl PaneGeometry {
    pub(crate) fn contains(self, x: f32, y: f32) -> bool {
        (self.x..self.x + self.width).contains(&x) && (self.y..self.y + self.height).contains(&y)
    }
}

/// Un borde compartido no pertenece a dos vistas. `contains` usa el extremo
/// superior inclusivo e inferior exclusivo, por lo que el siguiente panel es
/// el único receptor al cruzar una división.
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn document_pane_at(
    layouts: &[DocumentPaneLayout],
    x: f32,
    y: f32,
) -> Option<&DocumentPaneLayout> {
    layouts.iter().find(|layout| layout.geometry.contains(x, y))
}

/// Calcula una sola vez la geometría de las hojas visibles. Sus coordenadas
/// viven dentro del área documental, por eso el puntero de ventana debe
/// restar `DOCUMENT_VIEWPORT_TOP` antes de consultar `document_pane_at`.
pub(crate) fn document_pane_layouts_in(
    tree: &DocumentPaneTree,
    viewport: PaneGeometry,
) -> Vec<DocumentPaneLayout> {
    let mut layouts = Vec::with_capacity(tree.pane_count());
    tree.layout(
        PaneGeometry {
            x: viewport.x,
            y: viewport.y,
            width: viewport.width.max(0.0),
            height: viewport.height.max(0.0),
        },
        &mut layouts,
    );
    if layouts.len() > 1 {
        // Con varias hojas, cada una reserva su cabecera arriba. El contenido,
        // el foco y el scroll usan solo lo que queda debajo.
        for layout in &mut layouts {
            let header = PANE_HEADER_HEIGHT.min(layout.geometry.height);
            layout.geometry.y += header;
            layout.geometry.height -= header;
        }
    }
    layouts
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PaneHeaderHit {
    Mode(ModeTarget),
    Close,
    Title,
}

/// Cabecera de una hoja, en coordenadas del área documental.
pub(crate) fn pane_header_rect(geometry: PaneGeometry) -> PaneGeometry {
    PaneGeometry {
        x: geometry.x,
        y: geometry.y - PANE_HEADER_HEIGHT,
        width: geometry.width,
        height: PANE_HEADER_HEIGHT,
    }
}

pub(crate) fn pane_header_mode_x(header: PaneGeometry) -> Option<f32> {
    (header.width >= PANE_HEADER_MODE_MIN_WIDTH).then_some(
        header.x + header.width - PANE_HEADER_CLOSE_WIDTH - 8.0 - PANE_HEADER_MODE_SLOT * 3.0,
    )
}

/// Qué control de la cabecera hay bajo el puntero. Dibujo y clic comparten
/// esta misma descripción.
pub(crate) fn pane_header_hit(header: PaneGeometry, x: f32, y: f32) -> Option<PaneHeaderHit> {
    if !header.contains(x, y) {
        return None;
    }
    if x >= header.x + header.width - PANE_HEADER_CLOSE_WIDTH {
        return Some(PaneHeaderHit::Close);
    }
    if let Some(mode_x) = pane_header_mode_x(header)
        && (mode_x..mode_x + PANE_HEADER_MODE_SLOT * 3.0).contains(&x)
    {
        let target = match ((x - mode_x) / PANE_HEADER_MODE_SLOT) as usize {
            0 => ModeTarget::Reading,
            1 => ModeTarget::Editing,
            _ => ModeTarget::Compare,
        };
        return Some(PaneHeaderHit::Mode(target));
    }
    Some(PaneHeaderHit::Title)
}
