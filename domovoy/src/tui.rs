use crossterm::event::{self};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use std::io;
use yandex_home_sdk::Device;
mod dispatcher;
mod server;
use server::Server;
use yandex_home_sdk::color_mode;
mod color;
mod keys;
pub use color::*;
pub use dispatcher::*;

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
            let options = ColorOption::build(&mode);
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
