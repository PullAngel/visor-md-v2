//! Estado de hover y pulsación con transición. Es puro: recibe instantes y
//! devuelve cantidades entre 0 y 1; no dibuja ni despierta el bucle de
//! eventos. Reglas de movimiento de `docs/design.md`: el texto nunca se
//! mueve, solo cambian color y opacidad; lo que sale, sale más rápido que lo
//! que entra; con Reducir movimiento todo es instantáneo.

use crate::ui_target::UiTarget;
use std::time::{Duration, Instant};

/// Entrada del resaltado al pasar el puntero.
pub(crate) const HOVER_FADE_IN: Duration = Duration::from_millis(120);
/// Salida del resaltado: más corta que la entrada.
pub(crate) const HOVER_FADE_OUT: Duration = Duration::from_millis(80);
/// Duración del destello al pulsar.
pub(crate) const PRESS_FLASH: Duration = Duration::from_millis(160);

/// Avance suavizado (aceleración y frenado, sin rebote) de 0 a 1. Con
/// `reduce_motion` la transición ya terminó.
pub(crate) fn transition_progress(
    started: Instant,
    now: Instant,
    duration: Duration,
    reduce_motion: bool,
) -> f32 {
    if reduce_motion || duration.is_zero() {
        return 1.0;
    }
    let linear = (now.saturating_duration_since(started).as_secs_f32() / duration.as_secs_f32())
        .clamp(0.0, 1.0);
    linear * linear * (3.0 - 2.0 * linear)
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct UiMotion {
    /// Objetivo bajo el puntero y desde cuándo.
    hover: Option<(UiTarget, Instant)>,
    /// Objetivo que acaba de perder el puntero: desde cuándo y cuánto
    /// resaltado tenía, para que la salida parta de ahí.
    leaving: Option<(UiTarget, Instant, f32)>,
    /// Objetivo pulsado y cuándo.
    press: Option<(UiTarget, Instant)>,
}

// Se consume desde el dibujo en la fase 8 del plan de rediseño.
#[cfg_attr(not(test), allow(dead_code))]
impl UiMotion {
    /// Registra el objetivo bajo el puntero. Devuelve `true` si cambió.
    pub(crate) fn set_hover(
        &mut self,
        target: Option<UiTarget>,
        now: Instant,
        reduce_motion: bool,
    ) -> bool {
        if self.hover.map(|(current, _)| current) == target {
            return false;
        }
        if let Some((previous, _)) = self.hover {
            let amount = self.hover_amount(previous, now, reduce_motion);
            self.leaving = Some((previous, now, amount));
        }
        self.hover = target.map(|target| (target, now));
        true
    }

    pub(crate) fn press(&mut self, target: UiTarget, now: Instant) {
        self.press = Some((target, now));
    }

    /// Cuánto resaltado de hover tiene el objetivo, de 0 a 1.
    pub(crate) fn hover_amount(&self, target: UiTarget, now: Instant, reduce_motion: bool) -> f32 {
        if let Some((current, entered)) = self.hover
            && current == target
        {
            return transition_progress(entered, now, HOVER_FADE_IN, reduce_motion);
        }
        if let Some((previous, left, from)) = self.leaving
            && previous == target
        {
            let progress = transition_progress(left, now, HOVER_FADE_OUT, reduce_motion);
            return from * (1.0 - progress);
        }
        0.0
    }

    /// Intensidad del destello de pulsación: 1 al pulsar, 0 al terminar. Con
    /// Reducir movimiento no hay destello.
    pub(crate) fn press_amount(&self, target: UiTarget, now: Instant, reduce_motion: bool) -> f32 {
        match self.press {
            Some((pressed, started)) if pressed == target && !reduce_motion => {
                1.0 - transition_progress(started, now, PRESS_FLASH, false)
            }
            _ => 0.0,
        }
    }

    /// Hay alguna transición sin terminar.
    pub(crate) fn is_animating(&self, now: Instant, reduce_motion: bool) -> bool {
        if reduce_motion {
            return false;
        }
        let hovering = self
            .hover
            .is_some_and(|(_, entered)| now.saturating_duration_since(entered) < HOVER_FADE_IN);
        let leaving = self
            .leaving
            .is_some_and(|(_, left, _)| now.saturating_duration_since(left) < HOVER_FADE_OUT);
        let pressing = self
            .press
            .is_some_and(|(_, started)| now.saturating_duration_since(started) < PRESS_FLASH);
        hovering || leaving || pressing
    }

    /// Próximo cuadro que hace falta, o `None` si no hay nada animándose.
    pub(crate) fn next_frame_at(
        &self,
        now: Instant,
        frame: Duration,
        reduce_motion: bool,
    ) -> Option<Instant> {
        self.is_animating(now, reduce_motion).then(|| now + frame)
    }

    /// Olvida las transiciones terminadas.
    pub(crate) fn settle(&mut self, now: Instant) {
        if self
            .leaving
            .is_some_and(|(_, left, _)| now.saturating_duration_since(left) >= HOVER_FADE_OUT)
        {
            self.leaving = None;
        }
        if self
            .press
            .is_some_and(|(_, started)| now.saturating_duration_since(started) >= PRESS_FLASH)
        {
            self.press = None;
        }
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use crate::{AppAction, ModeTarget};

    const A: UiTarget = UiTarget::Toolbar(AppAction::Save);
    const B: UiTarget = UiTarget::ModeSegment(ModeTarget::Reading);

    fn despues(inicio: Instant, ms: u64) -> Instant {
        inicio + Duration::from_millis(ms)
    }

    #[test]
    fn el_progreso_va_de_cero_a_uno_sin_rebote() {
        let t0 = Instant::now();
        let d = Duration::from_millis(100);
        assert_eq!(transition_progress(t0, t0, d, false), 0.0);
        assert_eq!(transition_progress(t0, despues(t0, 100), d, false), 1.0);
        assert_eq!(transition_progress(t0, despues(t0, 500), d, false), 1.0);
        let mitad = transition_progress(t0, despues(t0, 50), d, false);
        assert!((mitad - 0.5).abs() < 1e-6, "{mitad}");
        let mut anterior = 0.0;
        for ms in 0..=100 {
            let valor = transition_progress(t0, despues(t0, ms), d, false);
            assert!(valor >= anterior && valor <= 1.0, "{ms} ms: {valor}");
            anterior = valor;
        }
    }

    #[test]
    fn con_reducir_movimiento_el_progreso_ya_termino() {
        let t0 = Instant::now();
        assert_eq!(transition_progress(t0, t0, HOVER_FADE_IN, true), 1.0);
        assert_eq!(transition_progress(t0, t0, Duration::ZERO, false), 1.0);
    }

    #[test]
    fn el_hover_entra_y_sale_de_forma_gradual_y_la_salida_es_mas_rapida() {
        let t0 = Instant::now();
        let mut motion = UiMotion::default();
        assert!(motion.set_hover(Some(A), t0, false));
        assert!(!motion.set_hover(Some(A), despues(t0, 10), false));
        assert_eq!(motion.hover_amount(A, t0, false), 0.0);
        let entrando = motion.hover_amount(A, despues(t0, 60), false);
        assert!(entrando > 0.0 && entrando < 1.0, "{entrando}");
        assert_eq!(motion.hover_amount(A, despues(t0, 120), false), 1.0);
        assert_eq!(motion.hover_amount(B, despues(t0, 120), false), 0.0);

        // Al salir parte del resaltado que tenía y llega a 0 antes de lo que
        // tardó en entrar.
        let salida = despues(t0, 200);
        assert!(motion.set_hover(Some(B), salida, false));
        assert_eq!(motion.hover_amount(A, salida, false), 1.0);
        let saliendo = motion.hover_amount(A, despues(salida, 40), false);
        assert!(saliendo > 0.0 && saliendo < 1.0, "{saliendo}");
        assert_eq!(motion.hover_amount(A, despues(salida, 80), false), 0.0);
        assert!(HOVER_FADE_OUT < HOVER_FADE_IN);
        // El nuevo objetivo empieza a entrar.
        assert_eq!(motion.hover_amount(B, salida, false), 0.0);
        assert!(motion.hover_amount(B, despues(salida, 60), false) > 0.0);
    }

    #[test]
    fn salir_a_mitad_de_la_entrada_parte_del_resaltado_alcanzado() {
        let t0 = Instant::now();
        let mut motion = UiMotion::default();
        motion.set_hover(Some(A), t0, false);
        let a_mitad = despues(t0, 60);
        let alcanzado = motion.hover_amount(A, a_mitad, false);
        motion.set_hover(None, a_mitad, false);
        assert_eq!(motion.hover_amount(A, a_mitad, false), alcanzado);
        assert!(motion.hover_amount(A, despues(a_mitad, 40), false) < alcanzado);
    }

    #[test]
    fn el_destello_de_pulsacion_decae_y_no_existe_con_reducir_movimiento() {
        let t0 = Instant::now();
        let mut motion = UiMotion::default();
        motion.press(A, t0);
        assert_eq!(motion.press_amount(A, t0, false), 1.0);
        let medio = motion.press_amount(A, despues(t0, 80), false);
        assert!(medio > 0.0 && medio < 1.0, "{medio}");
        assert_eq!(motion.press_amount(A, despues(t0, 160), false), 0.0);
        assert_eq!(motion.press_amount(B, t0, false), 0.0);
        assert_eq!(motion.press_amount(A, t0, true), 0.0);
    }

    #[test]
    fn solo_pide_cuadros_mientras_hay_una_transicion_sin_terminar() {
        let t0 = Instant::now();
        let cuadro = Duration::from_millis(16);
        let mut motion = UiMotion::default();
        assert_eq!(motion.next_frame_at(t0, cuadro, false), None);
        motion.set_hover(Some(A), t0, false);
        assert_eq!(
            motion.next_frame_at(despues(t0, 50), cuadro, false),
            Some(despues(t0, 66))
        );
        assert_eq!(motion.next_frame_at(despues(t0, 130), cuadro, false), None);
        motion.press(A, despues(t0, 200));
        assert!(motion.is_animating(despues(t0, 250), false));
        assert!(!motion.is_animating(despues(t0, 400), false));
        // Con Reducir movimiento nunca hay nada que animar.
        assert_eq!(motion.next_frame_at(despues(t0, 210), cuadro, true), None);
    }

    #[test]
    fn settle_olvida_lo_terminado() {
        let t0 = Instant::now();
        let mut motion = UiMotion::default();
        motion.set_hover(Some(A), t0, false);
        motion.set_hover(None, despues(t0, 200), false);
        motion.press(A, despues(t0, 200));
        motion.settle(despues(t0, 500));
        assert!(motion.leaving.is_none());
        assert!(motion.press.is_none());
    }
}
