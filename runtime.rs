// Independent Horizon native zoom mod. MIT licensed.
// Built as crate::game::infinite_zoom by horizon-mixins; this same file is
// a standalone library to unit-test its math with `cargo test`.
use std::cell::RefCell;

pub const ID: &str = "infinite_zoom";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Hold C and scroll for practically unlimited zoom, with HUD and precision controls.";
pub const KEY: &str = "zoom";
pub const DEFAULT_KEY: &str = "key.keyboard.c";
pub const ENGINE_MAX: f32 = 1_000_000.0;
const MIN: f32 = 1.0;
const STARTS: [u32; 8] = [2, 4, 8, 16, 32, 64, 128, 256];
const CAPS: [u32; 5] = [100, 1_000, 10_000, 100_000, 1_000_000];
const STEPS: [f32; 5] = [1.10, 1.25, 1.50, 2.0, 4.0];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    pub start: u32,
    pub cap: u32,
    pub step_index: usize,
    pub smooth: bool,
    pub sensitivity: bool,
    pub hud: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            start: 8,
            cap: 1_000_000,
            step_index: 2,
            smooth: true,
            sensitivity: true,
            hud: true,
        }
    }
}

impl Settings {
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let default = Self::default();
        let number = |key: &str| get(key).and_then(|v| v.trim().parse::<u32>().ok());
        let flag = |key: &str, fallback: bool| {
            get(key).map_or(fallback, |v| v.trim() != "false")
        };
        Self {
            start: number("start")
                .filter(|v| STARTS.contains(v))
                .unwrap_or(default.start),
            cap: number("cap")
                .filter(|v| CAPS.contains(v))
                .unwrap_or(default.cap),
            step_index: number("step")
                .map(|v| v as usize)
                .filter(|&i| i < STEPS.len())
                .unwrap_or(default.step_index),
            smooth: flag("smooth", default.smooth),
            sensitivity: flag("sensitivity", default.sensitivity),
            hud: flag("hud", default.hud),
        }
    }

    pub fn labels(&self) -> Vec<String> {
        let on = |v: bool| if v { "ON" } else { "OFF" };
        vec![
            format!("Starting zoom: {}x", self.start),
            format!("Max zoom: {}x", self.cap),
            format!("Wheel multiplier: {:.2}x", STEPS[self.step_index]),
            format!("Smooth zoom: {}", on(self.smooth)),
            format!("Precision mouse: {}", on(self.sensitivity)),
            format!("Zoom HUD: {}", on(self.hud)),
        ]
    }

    pub fn click(&mut self, button: usize) {
        match button {
            0 => {
                let index = STARTS.iter().position(|&v| v == self.start).unwrap_or(0);
                self.start = STARTS[(index + 1) % STARTS.len()];
            }
            1 => {
                let index = CAPS.iter().position(|&v| v == self.cap).unwrap_or(0);
                self.cap = CAPS[(index + 1) % CAPS.len()];
            }
            2 => self.step_index = (self.step_index + 1) % STEPS.len(),
            3 => self.smooth = !self.smooth,
            4 => self.sensitivity = !self.sensitivity,
            5 => self.hud = !self.hud,
            _ => {}
        }
    }

    pub fn stored(&self) -> [(&'static str, String); 6] {
        let flag = |v: bool| if v { "true" } else { "false" }.to_string();
        [
            ("start", self.start.to_string()),
            ("cap", self.cap.to_string()),
            ("step", self.step_index.to_string()),
            ("smooth", flag(self.smooth)),
            ("sensitivity", flag(self.sensitivity)),
            ("hud", flag(self.hud)),
        ]
    }
}

#[derive(Default)]
struct State {
    settings: Option<Settings>,
    held: Option<f32>,
}
thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    STATE.with_borrow_mut(|state| *state.settings.get_or_insert_with(|| Settings::read(get)))
}

pub fn set_settings(value: Settings) {
    STATE.with_borrow_mut(|s| {
        s.settings = Some(value);
        if let Some(held) = &mut s.held {
            *held = held.clamp(MIN, value.cap as f32);
        }
    });
}

pub fn release() {
    STATE.with_borrow_mut(|state| state.held = None);
}

/// Hold to zoom. Don't engage while navigating menus.
pub fn key(playing: bool, pressed: bool, get: impl Fn(&str) -> Option<String>) {
    if !pressed {
        release();
        return;
    }
    if !playing {
        return;
    }
    let cfg = settings(get);
    STATE.with_borrow_mut(|state| {
        state.held = Some((cfg.start as f32).clamp(MIN, cfg.cap as f32));
    });
}

/// Returns true if wheel input was consumed by zoom rather than hotbar.
pub fn wheel(steps: f32) -> bool {
    STATE.with_borrow_mut(|state| {
        let cfg = state.settings.unwrap_or_default();
        let Some(factor) = &mut state.held else {
            return false;
        };
        if !steps.is_finite() {
            return true;
        }
        let multiplier = STEPS[cfg.step_index].powf(steps.clamp(-64.0, 64.0));
        *factor = (*factor * multiplier).clamp(MIN, cfg.cap as f32);
        true
    })
}

/// Game's native_zoom hook expects factor, smooth and mouse turning ratio.
pub fn view() -> Option<(f32, bool, f32)> {
    STATE.with_borrow(|s| {
        let factor = s.held?;
        let cfg = s.settings?;
        let turn = if cfg.sensitivity { 1.0 / factor } else { 1.0 };
        Some((factor, cfg.smooth, turn))
    })
}

pub fn hud_label() -> Option<String> {
    STATE.with_borrow(|s| {
        let factor = s.held?;
        if !s.settings?.hud || factor <= 1.0 {
            return None;
        }
        Some(format!("ZOOM {:.0}x", factor))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty(_key: &str) -> Option<String> {
        None
    }

    #[test]
    fn defaults_are_valid() {
        let s = Settings::read(empty);
        assert_eq!(s.start, 8);
        assert_eq!(s.cap, 1_000_000);
        assert_eq!(s.step_index, 2);
        assert!(s.hud);
    }

    #[test]
    fn settings_round_trip_and_cycle() {
        let mut s = Settings::default();
        s.click(0);
        s.click(1);
        s.click(2);
        s.click(3);
        s.click(4);
        s.click(5);
        let stored = s.stored();
        let got = Settings::read(|key| {
            stored.iter().find(|(k, _)| *k == key).map(|(_, v)| v.clone())
        });
        assert_eq!(s, got);
    }

    #[test]
    fn zooms_beyond_old_engine_limit_and_stops_at_million() {
        key(true, true, empty);
        assert_eq!(view().map(|v| v.0), Some(8.0));
        assert!(wheel(30.0));
        assert!(view().unwrap().0 > 10_000.0);
        assert!(wheel(64.0));
        assert_eq!(view().unwrap().0, ENGINE_MAX);
        assert!(hud_label().unwrap().contains("1000000x"));
        key(false, false, empty);
        assert_eq!(view(), None);
        assert!(!wheel(1.0));
    }

    #[test]
    fn ignores_invalid_wheel_and_unfocused_key() {
        release();
        key(false, true, empty);
        assert_eq!(view(), None);
        key(true, true, empty);
        assert!(wheel(f32::NAN));
        assert_eq!(view().unwrap().0, 8.0);
        release();
    }
}
