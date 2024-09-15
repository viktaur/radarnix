use std::io;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{layout::{Alignment, Constraint, Layout}, style::{Color, Modifier, Style, Stylize}, symbols::border, widgets::{block::Title, canvas::{Canvas, Map, MapResolution}, Block, Borders, List, ListItem, ListState, ListDirection, Padding, Paragraph}, CompletedFrame, DefaultTerminal, Frame};

use crate::{api::ApiProvider, App};

pub struct ApiProvidersWidget {
    items: Vec<ApiProvider>,
    state: ListState,
}

// impl FromIterator<&'static str> for ApiProvidersWidget {
//     fn from_iter<T: IntoIterator<Item = &'static str>>(iter: T) -> Self {
//         let items = iter.into_iter().collect();
//         let state = ListState::default();
//         ApiProvidersWidget { items, state }
//     }
// }

impl App {
    /// Renders a frame to the terminal containing the menu scene.
    pub(crate) fn render_menu_scene<'a>(
        &'a self,
        terminal: &'a mut DefaultTerminal
    ) -> io::Result<CompletedFrame<'_>> {
        terminal.draw(|frame| {
            self.draw_borders(frame);
            self.draw_menu(frame);
        })
    }

    /// Renders a frame to the terminal contianing the explorer scence.
    pub(crate) fn render_explorer_scene<'a>(
        &'a self,
        terminal: &'a mut DefaultTerminal
    ) -> io::Result<CompletedFrame<'_>> {
        terminal.draw(|frame| {
            self.draw_borders(frame);
            self.draw_map(frame);
        })
    }

    /// Renders a widget to a frame with the world map
    pub(crate) fn draw_map(&self, frame: &mut Frame) {
        let block = Block::new()
            .borders(Borders::empty())
            .padding(Padding::uniform(1));

        let canvas = Canvas::default()
            .marker(self.marker)
            .block(block)
            .paint(|ctx| {
                ctx.draw(&Map {
                    color: Color::Green,
                    resolution: MapResolution::High,
                });
            })
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0]);

        frame.render_widget(canvas, frame.area())
    }

    /// Renders a widget to a frame with the main menu.
    pub(crate) fn draw_menu(&self, frame: &mut Frame) {
        let [title_area, main_area] = Layout::vertical([
            Constraint::Length(20),
            Constraint::Length(20),
        ])
        .margin(1)
        .areas(frame.area());

        // frame.render_widget(Block::bordered().title("Title Bar"), title_area);
        // frame.render_widget(Block::bordered().title("Main area"), main_area);

        let greeting = Paragraph::new(
            "Welcome to Radarnix! Please select an API provider and introduce your key"
        ).green();

        let list = List::new(ApiProvider::get_list())
            .block(Block::bordered().title("Select an API provider"))
            .style(Style::default().fg(Color::Yellow))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);

        frame.render_widget(greeting, title_area);
        frame.render_widget(list, main_area);
    }

    /// Renders a widget to a frame with the borders and application title, present in all scenes.
    pub(crate) fn draw_borders(&self, frame: &mut Frame) {
        let title = Title::from("  ✈ R A D A R N I X ✈  ".bold().green());
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .border_set(border::THICK)
            .green();

        frame.render_widget(block, frame.area());
    }
}
