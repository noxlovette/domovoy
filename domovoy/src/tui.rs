use crossterm::event::{self};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use std::io;
use yandex_home_sdk::{
    ColorModel, ColorMode, ColorSettingParameters, Device, DeviceCapability, Group,
    GroupCapability, OnOffState,
};
mod dispatcher;
mod server;
use server::Server;
mod color;
mod keys;
pub use color::*;
pub use dispatcher::*;

pub(crate) enum Entry {
    Light {
        id: String,
        name: String,
        options: Vec<ColorOption>,
        is_on: Option<bool>,
    },
    Group {
        id: String,
        name: String,
        options: Vec<ColorOption>,
        is_on: Option<bool>,
    },
}

impl Entry {
    pub(crate) fn name(&self) -> &str {
        match self {
            Entry::Light { name, .. } | Entry::Group { name, .. } => name,
        }
    }

    pub(crate) fn id(&self) -> &str {
        match self {
            Entry::Light { id, .. } | Entry::Group { id, .. } => id,
        }
    }

    pub(crate) fn options(&self) -> &[ColorOption] {
        match self {
            Entry::Light { options, .. } | Entry::Group { options, .. } => options,
        }
    }

    pub(crate) fn is_on(&self) -> Option<bool> {
        match self {
            Entry::Light { is_on, .. } | Entry::Group { is_on, .. } => *is_on,
        }
    }

    pub(crate) fn set_is_on(&mut self, v: bool) {
        match self {
            Entry::Light { is_on, .. } | Entry::Group { is_on, .. } => *is_on = Some(v),
        }
    }

    pub(crate) fn is_group(&self) -> bool {
        matches!(self, Entry::Group { .. })
    }
}

#[derive(Copy, Clone)]
pub(crate) enum Screen {
    Main,
    ColorPicker { entry_idx: usize },
}

pub(crate) struct App {
    pub(crate) exit: bool,
    pub(crate) screen: Screen,
    pub(crate) entries: Vec<Entry>,
    pub(crate) entry_list: ListState,
    pub(crate) color_list: ListState,
    pub(crate) status: String,
    pub(crate) server: Server,
}

pub fn run() -> anyhow::Result<()> {
    let server = Server::new()?;

    let (devices, groups) = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(server.light_info())
    })?;

    let mut entries: Vec<Entry> = Vec::new();

    for d in devices {
        let is_on = extract_device_is_on(&d);
        if let Some(mode) = color_mode_from_device(&d) {
            let options = ColorOption::build(&mode);
            if !options.is_empty() {
                entries.push(Entry::Light {
                    id: d.id,
                    name: d.name,
                    options,
                    is_on,
                });
            }
        }
    }

    for g in groups {
        let is_on = extract_group_is_on(&g);
        if let Some(mode) = color_mode_from_group(&g) {
            let options = ColorOption::build(&mode);
            if !options.is_empty() {
                entries.push(Entry::Group {
                    id: g.id,
                    name: g.name,
                    options,
                    is_on,
                });
            }
        }
    }

    let (entry_list, status) = if entries.is_empty() {
        (
            ListState::default(),
            "No controllable colour lights or groups found".to_string(),
        )
    } else {
        (
            ListState::default().with_selected(Some(0)),
            "j/k · Enter colour · o on · O off · q quit".to_string(),
        )
    };

    let mut app = App {
        exit: false,
        screen: Screen::Main,
        entries,
        entry_list,
        color_list: ListState::default().with_selected(Some(0)),
        status,
        server,
    };

    ratatui::run(|terminal| app.run_loop(terminal))?;
    Ok(())
}

fn color_mode_from_params(params: &ColorSettingParameters) -> Option<ColorMode> {
    Some(match (&params.color_model, &params.temperature_k) {
        (Some(ColorModel::Rgb), Some(r)) => ColorMode::RgbAndTemperature(r.clone()),
        (Some(ColorModel::Rgb), None) => ColorMode::Rgb,
        (Some(ColorModel::Hsv), Some(r)) => ColorMode::HsvAndTemperature(r.clone()),
        (Some(ColorModel::Hsv), None) => ColorMode::Hsv,
        (_, Some(r)) => ColorMode::Temperature(r.clone()),
        _ => return None,
    })
}

fn color_mode_from_device(device: &Device) -> Option<ColorMode> {
    let params = device.capabilities.iter().find_map(|c| match c {
        DeviceCapability::ColorSetting { parameters, .. } => Some(parameters),
        _ => None,
    })?;
    color_mode_from_params(params)
}

fn color_mode_from_group(group: &Group) -> Option<ColorMode> {
    let params = group.capabilities.iter().find_map(|c| match c {
        GroupCapability::ColorSetting { parameters, .. } => Some(parameters),
        _ => None,
    })?;
    color_mode_from_params(params)
}

fn extract_device_is_on(device: &Device) -> Option<bool> {
    device.capabilities.iter().find_map(|c| {
        if let DeviceCapability::OnOff {
            state: Some(OnOffState::On(on)),
            ..
        } = c
        {
            Some(*on)
        } else {
            None
        }
    })
}

fn extract_group_is_on(group: &Group) -> Option<bool> {
    group.capabilities.iter().find_map(|c| {
        if let GroupCapability::OnOff {
            state: Some(OnOffState::On(on)),
            ..
        } = c
        {
            Some(*on)
        } else {
            None
        }
    })
}

impl App {
    fn run_loop(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            if let Some(key) = event::read()?.as_key_press_event() {
                match self.screen {
                    Screen::Main => self.handle_device_keys(key.code),
                    Screen::ColorPicker { .. } => self.handle_color_keys(key.code),
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        match self.screen {
            Screen::Main => self.draw_main(frame),
            Screen::ColorPicker { entry_idx } => self.draw_color_picker(frame, entry_idx),
        }
    }

    fn draw_main(&mut self, frame: &mut Frame) {
        let [main, status_bar] = frame.area().layout(&Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
        ]));

        let items: Vec<ListItem> = self
            .entries
            .iter()
            .map(|e| {
                let prefix = if e.is_group() { "[G] " } else { "    " };
                let on_mark = match e.is_on() {
                    Some(true) => " ●",
                    Some(false) => " ○",
                    None => "",
                };
                ListItem::new(format!("{}{}{}", prefix, e.name(), on_mark))
            })
            .collect();

        let list = List::new(items)
            .block(Block::bordered().title(" Domovoy · Lights & Groups "))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, main, &mut self.entry_list);
        frame.render_widget(
            Paragraph::new(self.status.as_str()).style(Style::new().add_modifier(Modifier::DIM)),
            status_bar,
        );
    }

    fn draw_color_picker(&mut self, frame: &mut Frame, entry_idx: usize) {
        let [main, status_bar] = frame.area().layout(&Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
        ]));

        let entry = &self.entries[entry_idx];
        let kind = if entry.is_group() { "group" } else { "light" };
        let title = format!(" {} [{}] · Pick a colour ", entry.name(), kind);

        let items: Vec<ListItem> = entry
            .options()
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
