use crate::tui::dispatcher::ColorAction;
use ratatui::style::Color;
use yandex_home_sdk::ColorMode;

pub fn rgb_to_hsv(rgb: u32) -> (u16, u8, u8) {
    let r = ((rgb >> 16) & 0xFF) as f32 / 255.0;
    let g = ((rgb >> 8) & 0xFF) as f32 / 255.0;
    let b = (rgb & 0xFF) as f32 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta).rem_euclid(6.0))
    } else if max == g {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };
    let s = if max == 0.0 { 0.0 } else { delta / max };
    (h as u16, (s * 100.0) as u8, (max * 100.0) as u8)
}

// ── static presets ────────────────────────────────────────────────────────────

pub const RGB_PRESETS: &[(&str, u32, Color)] = &[
    ("Red", 0xFF0000, Color::Red),
    ("Orange", 0xFF8000, Color::Rgb(255, 128, 0)),
    ("Yellow", 0xFFFF00, Color::Yellow),
    ("Green", 0x00FF00, Color::Green),
    ("Cyan", 0x00FFFF, Color::Cyan),
    ("Blue", 0x0000FF, Color::Blue),
    ("Purple", 0x8000FF, Color::Rgb(128, 0, 255)),
    ("Pink", 0xFF00FF, Color::Magenta),
    ("White", 0xFFFFFF, Color::White),
];

pub const TEMP_PRESETS: &[(&str, u32)] = &[
    ("Candle      (1900 K)", 1900),
    ("Warm White  (2700 K)", 2700),
    ("Soft White  (3000 K)", 3000),
    ("Neutral     (3500 K)", 3500),
    ("Cool White  (4500 K)", 4500),
    ("Daylight    (5600 K)", 5600),
    ("Cool Day    (6500 K)", 6500),
];

fn kelvin_swatch(k: u32) -> Color {
    match k {
        ..=2000 => Color::Rgb(255, 147, 41),
        ..=2700 => Color::Rgb(255, 197, 143),
        ..=3500 => Color::Rgb(255, 228, 206),
        ..=5000 => Color::White,
        _ => Color::Rgb(201, 226, 255),
    }
}

#[derive(Clone)]
pub struct ColorOption {
    pub label: &'static str,
    pub swatch: Color,
    pub action: ColorAction,
}

impl ColorOption {
    pub fn build(mode: &ColorMode) -> Vec<Self> {
        let mut opts = Vec::new();

        match mode {
            ColorMode::Rgb | ColorMode::RgbAndTemperature(_) => {
                for &(label, value, swatch) in RGB_PRESETS {
                    opts.push(ColorOption {
                        label,
                        swatch,
                        action: ColorAction::Rgb(value),
                    });
                }
            }
            ColorMode::Hsv | ColorMode::HsvAndTemperature(_) => {
                for &(label, value, swatch) in RGB_PRESETS {
                    let (h, s, v) = rgb_to_hsv(value);
                    opts.push(ColorOption {
                        label,
                        swatch,
                        action: ColorAction::Hsv { h, s, v },
                    });
                }
            }
            _ => {}
        }

        let range = match mode {
            ColorMode::Temperature(r)
            | ColorMode::RgbAndTemperature(r)
            | ColorMode::HsvAndTemperature(r) => Some(r),
            _ => None,
        };
        if let Some(r) = range {
            for &(label, kelvin) in TEMP_PRESETS {
                if kelvin >= r.min && kelvin <= r.max {
                    opts.push(ColorOption {
                        label,
                        swatch: kelvin_swatch(kelvin),
                        action: ColorAction::Temperature(kelvin),
                    });
                }
            }
        }
        opts
    }
}
