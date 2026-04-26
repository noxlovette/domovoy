use crate::tui::{App, Screen};
use crossterm::event::KeyCode;

impl App {
    pub fn handle_device_keys(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('j') | KeyCode::Down => self.entry_list.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.entry_list.select_previous(),
            KeyCode::Enter => {
                if let Some(idx) = self.entry_list.selected() {
                    self.color_list.select(Some(0));
                    self.screen = Screen::ColorPicker { entry_idx: idx };
                }
            }
            KeyCode::Char('o') => self.send_power(true),
            KeyCode::Char('O') => self.send_power(false),
            KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
            _ => {}
        }
    }

    fn send_power(&mut self, on: bool) {
        let Some(idx) = self.entry_list.selected() else {
            return;
        };
        let entry = &self.entries[idx];
        let id = entry.id().to_string();
        let name = entry.name().to_string();
        let is_group = entry.is_group();

        let result = {
            let server = &self.server;
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async {
                    if is_group {
                        server.toggle_group(&id, on).await
                    } else {
                        server.toggle_device(&id, on).await
                    }
                })
            })
        };

        match result {
            Ok(()) => {
                self.entries[idx].set_is_on(on);
                self.status = format!("{} → {}", name, if on { "on" } else { "off" });
            }
            Err(e) => self.status = format!("Error: {e}"),
        }
    }

    pub fn handle_color_keys(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char('j') | KeyCode::Down => self.color_list.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.color_list.select_previous(),
            KeyCode::Esc => self.screen = Screen::Main,
            KeyCode::Enter => {
                if let Screen::ColorPicker { entry_idx } = self.screen {
                    if let Some(color_idx) = self.color_list.selected() {
                        let entry = &self.entries[entry_idx];
                        let opt = entry.options()[color_idx].clone();
                        let id = entry.id().to_string();
                        let name = entry.name().to_string();
                        let is_group = entry.is_group();

                        let result = {
                            let server = &self.server;
                            tokio::task::block_in_place(|| {
                                tokio::runtime::Handle::current().block_on(async {
                                    if is_group {
                                        server.set_group_color(&id, opt.action.clone()).await
                                    } else {
                                        server.set_color(&id, opt.action.clone()).await
                                    }
                                })
                            })
                        };

                        self.status = match result {
                            Ok(()) => format!("{name} → {}", opt.label),
                            Err(e) => format!("Error: {e}"),
                        };
                        self.screen = Screen::Main;
                    }
                }
            }
            KeyCode::Char('q') => self.exit = true,
            _ => {}
        }
    }
}
