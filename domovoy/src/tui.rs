use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use std::io;
use yandex_home_sdk::Device;

mod dispatcher;
mod server;
use server::{ColorAction, ColorMode, Server, color_mode};

fn rgb_to_hsv(rgb: u32) -> (u16, u8, u8) {
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

const RGB_PRESETS: &[(&str, u32, Color)] = &[
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

const TEMP_PRESETS: &[(&str, u32)] = &[
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

// ── colour options (per-device, built at startup) ─────────────────────────────

#[derive(Clone)]
struct ColorOption {
    label: &'static str,
    swatch: Color,
    action: ColorAction,
}

fn build_options(mode: &ColorMode) -> Vec<ColorOption> {
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

// ── app ───────────────────────────────────────────────────────────────────────

struct LightEntry {
    device: Device,
    options: Vec<ColorOption>,
}

#[derive(Copy, Clone)]
enum Screen {
    Devices,
    ColorPicker { device_idx: usize },
}

struct App {
    exit: bool,
    screen: Screen,
    lights: Vec<LightEntry>,
    device_list: ListState,
    color_list: ListState,
    status: String,
    server: Server,
}

pub fn run() -> anyhow::Result<()> {
    let server = Server::new()?;

    let devices = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(server.light_devices())
    })?;

    let lights: Vec<LightEntry> = devices
        .into_iter()
        .filter_map(|d| {
            let mode = color_mode(&d)?;
            let options = build_options(&mode);
            if options.is_empty() {
                return None;
            }
            Some(LightEntry { device: d, options })
        })
        .collect();

    let (device_list, status) = if lights.is_empty() {
        (
            ListState::default(),
            "No controllable colour lights found".to_string(),
        )
    } else {
        (
            ListState::default().with_selected(Some(0)),
            "j/k navigate · Enter set colour · q quit".to_string(),
        )
    };

    let mut app = App {
        exit: false,
        screen: Screen::Devices,
        lights,
        device_list,
        color_list: ListState::default().with_selected(Some(0)),
        status,
        server,
    };

    ratatui::run(|terminal| app.run_loop(terminal))?;
    Ok(())
}

impl App {
    fn run_loop(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            if let Some(key) = event::read()?.as_key_press_event() {
                match self.screen {
                    Screen::Devices => self.handle_device_keys(key.code),
                    Screen::ColorPicker { .. } => self.handle_color_keys(key.code),
                }
            }
        }
        Ok(())
    }

    fn handle_device_keys(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('j') | KeyCode::Down => self.device_list.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.device_list.select_previous(),
            KeyCode::Enter => {
                if let Some(idx) = self.device_list.selected() {
                    self.color_list.select(Some(0));
                    self.screen = Screen::ColorPicker { device_idx: idx };
                }
            }
            KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
            _ => {}
        }
    }

    fn handle_color_keys(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('j') | KeyCode::Down => self.color_list.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.color_list.select_previous(),
            KeyCode::Esc => self.screen = Screen::Devices,
            KeyCode::Enter => {
                if let Screen::ColorPicker { device_idx } = self.screen {
                    if let Some(color_idx) = self.color_list.selected() {
                        let entry = &self.lights[device_idx];
                        let opt = entry.options[color_idx].clone();
                        let device_id = entry.device.id.clone();
                        let device_name = entry.device.name.clone();

                        let result = {
                            let server = &self.server;
                            tokio::task::block_in_place(|| {
                                tokio::runtime::Handle::current()
                                    .block_on(server.set_color(&device_id, opt.action.clone()))
                            })
                        };

                        self.status = match result {
                            Ok(()) => format!("{device_name} → {}", opt.label),
                            Err(e) => format!("Error: {e}"),
                        };
                        self.screen = Screen::Devices;
                    }
                }
            }
            KeyCode::Char('q') => self.exit = true,
            _ => {}
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        match self.screen {
            Screen::Devices => self.draw_devices(frame),
            Screen::ColorPicker { device_idx } => self.draw_color_picker(frame, device_idx),
        }
    }

    fn draw_devices(&mut self, frame: &mut Frame) {
        let [main, status_bar] = frame.area().layout(&Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
        ]));

        let items: Vec<ListItem> = self
            .lights
            .iter()
            .map(|e| ListItem::new(e.device.name.as_str()))
            .collect();

        let list = List::new(items)
            .block(Block::bordered().title(" Domovoy · Lights "))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, main, &mut self.device_list);
        frame.render_widget(
            Paragraph::new(self.status.as_str()).style(Style::new().add_modifier(Modifier::DIM)),
            status_bar,
        );
    }

    fn draw_color_picker(&mut self, frame: &mut Frame, device_idx: usize) {
        let [main, status_bar] = frame.area().layout(&Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
        ]));

        let entry = &self.lights[device_idx];
        let title = format!(" {} · Pick a colour ", entry.device.name);

        let items: Vec<ListItem> = entry
            .options
            .iter()
            .map(|opt| {
                ListItem::new(Line::from(vec![
                    Span::styled("██ ", Style::new().fg(opt.swatch)),
                    Span::raw(opt.label),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::bordered().title(title))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, main, &mut self.color_list);
        frame.render_widget(
            Paragraph::new("Enter to apply · Esc to cancel")
                .style(Style::new().add_modifier(Modifier::DIM)),
            status_bar,
        );
    }
}
