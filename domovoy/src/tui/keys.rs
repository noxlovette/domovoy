use crate::tui::{App, Screen};
use crossterm::event::KeyCode;

impl App {
    pub fn handle_device_keys(&mut self, code: KeyCode) {
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

    pub fn handle_color_keys(&mut self, code: KeyCode) {
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
}
