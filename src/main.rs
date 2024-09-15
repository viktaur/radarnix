use api::{ApiClient, ApiProvider};
use crossterm::event::{Event, KeyEvent};
use ratatui::{
    buffer::Buffer, crossterm::event::{self, KeyCode, KeyEventKind}, layout::{Alignment, Constraint, Layout, Rect}, style::{palette::tailwind::BLUE, Color, Modifier, Style, Stylize}, symbols::{border, Marker}, text::{Line, Text}, widgets::{
        block::{Position, Title}, canvas::{Canvas, Circle, Map, MapResolution, Rectangle}, Block, Borders, List, ListDirection, Padding, Paragraph, Widget
    }, CompletedFrame, DefaultTerminal, Frame
};
use std::io::{self, Stdout};

mod models;
mod api;
mod graphic;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}

pub enum Scene {
    Menu,
    Explorer
}

impl Scene {
    // TODO: move these methods back to App, since we need to read its state for selection.
    pub fn handle_input(&self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match self {
            Scene::Menu => {
                KeyCode

            },
            Scene::Explorer => todo!(),
        }
    }

    fn handle_input_menu(&self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => self.state.select_previous(),
            KeyCode::Down => self.state.select_next(),
            KeyCode::Home => self.state.select_first(),
            KeyCode::End => self.state.select_last(),
        }
    }

    fn handle_input_explorer(&self, key: KeyEvent) {

    }
}

struct App {
    marker: Marker,
    current_scene: Scene,
    api_client: Option<ApiClient>,
}

impl Default for App {
    fn default() -> Self {
        App {
            marker: Marker::Braille,
            current_scene: Scene::Menu,
            api_client: None,
        }
    }
}

impl App {
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        loop {
            match self.current_scene {
                Scene::Menu => self.render_menu_scene(&mut terminal)?,
                Scene::Explorer => self.render_explorer_scene(&mut terminal)?,
            };

            if let Event::Key(key) = event::read()? {
                self.current_scene.handle_input(key);
            }
        }
    }
}
