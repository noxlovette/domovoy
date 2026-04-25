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
use server::Server;

const COLORS: &[(&str, u32, Color)] = &[
    ("Red", 0xFF0000, Color::Red),
    ("Orange", 0xFF8000, Color::Rgb(255, 128, 0)),
    ("Yellow", 0xFFFF00, Color::Yellow),
    ("Green", 0x00FF00, Color::Green),
    ("Cyan", 0x00FFFF, Color::Cyan),
    ("Blue", 0x0000FF, Color::Blue),
    ("Purple", 0x8000FF, Color::Rgb(128, 0, 255)),
    ("Pink", 0xFF00FF, Color::Magenta),
    ("Warm White", 0xFFE0A0, Color::Rgb(255, 224, 160)),
    ("White", 0xFFFFFF, Color::White),
];

#[derive(Copy, Clone)]
enum Screen {
    Devices,
    ColorPicker { device_idx: usize },
}

struct App {
    exit: bool,
    screen: Screen,
    devices: Vec<Device>,
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

    let (device_list, status) = if devices.is_empty() {
        (
            ListState::default(),
            "No colour lights found".to_string(),
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
        devices,
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
            KeyCode::Esc => {
                self.screen = Screen::Devices;
            }
            KeyCode::Enter => {
                if let Screen::ColorPicker { device_idx } = self.screen {
                    if let Some(color_idx) = self.color_list.selected() {
                        let (color_name, rgb, _) = COLORS[color_idx];
                        let device_id = self.devices[device_idx].id.clone();
                        let device_name = self.devices[device_idx].name.clone();

                        let result = {
                            let server = &self.server;
                            tokio::task::block_in_place(|| {
                                tokio::runtime::Handle::current()
                                    .block_on(server.set_color(&device_id, rgb))
                            })
                        };

                        self.status = match result {
                            Ok(()) => format!("{device_name} → {color_name}"),
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
        let [main, status_bar] = frame.area().layout(
            &Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]),
        );

        let items: Vec<ListItem> = self
            .devices
            .iter()
            .map(|d| ListItem::new(d.name.as_str()))
            .collect();

        let list = List::new(items)
            .block(Block::bordered().title(" Domovoy · Lights "))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, main, &mut self.device_list);
        frame.render_widget(
            Paragraph::new(self.status.as_str())
                .style(Style::new().add_modifier(Modifier::DIM)),
            status_bar,
        );
    }

    fn draw_color_picker(&mut self, frame: &mut Frame, device_idx: usize) {
        let [main, status_bar] = frame.area().layout(
            &Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]),
        );

        let device_name = &self.devices[device_idx].name;
        let title = format!(" {} · Pick a colour ", device_name);

        let items: Vec<ListItem> = COLORS
            .iter()
            .map(|(name, _, color)| {
                ListItem::new(Line::from(vec![
                    Span::styled("██ ", Style::new().fg(*color)),
                    Span::raw(*name),
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
