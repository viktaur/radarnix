use std::io::{self, Stdout};

use crossterm::event::{Event, KeyEvent};
use ratatui::{
    buffer::Buffer, crossterm::event::{self, KeyCode, KeyEventKind}, layout::{Alignment, Rect}, style::Stylize, symbols::border, text::{Line, Text}, widgets::{block::{Position, Title}, Block, Paragraph, Widget}, DefaultTerminal, Frame
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
struct App {
    nav: NavState
}

#[derive(Debug)]
struct NavState {
    center_lat: f64,
    center_lon: f64,
    /// Represents how zoomed in the map is.
    ///
    /// At scale = 1.0 the entire world map should be
    /// visible, i.e. a full range of 360 deg for longitude and 180 deg for latitude. At
    /// scale = 2.0, only half the range should be shown, i.e. 180 deg for longitude and
    /// 90 deg for latitude, and so on.
    scale: f64
}

impl NavState {
    fn get_visible_lat_range(&self) -> f64 {
        180.0 / self.scale
    }

    fn get_visible_lon_range(&self) -> f64 {
        360.0 / self.scale
    }

    fn move_up(&mut self) {
        let offset = 5.0;
        self.center_lat = f64::min(90.0, self.center_lat + offset);
    }

    fn move_down(&mut self) {
        let offset = 5.0;
        self.center_lat = f64::max(-90.0, self.center_lat - offset);
    }

    fn move_left(&mut self) {
        let offset = 5.0;
        self.center_lon = f64::max(-180.0, self.center_lon - offset);
    }

    fn move_right(&mut self) {
        let offset = 5.0;
        self.center_lon = f64::min(180.0, self.center_lon + offset);
    }

    fn zoom_in(&mut self) {
        let factor = 1.1;
        let max_zoom = 100.0; // define
        self.scale = f64::min(max_zoom, self.scale * factor);
    }

    fn zoom_out(&mut self) {
        let factor = 1.1;
        let min_zoom = 1.0;
        self.scale = f64::max(min_zoom, self.scale / factor);
    }
}

impl Default for NavState {
    // Set default coordinates to LHR
    fn default() -> Self {
        NavState {
            center_lat: 51.468,
            center_lon: -0.45506,
            scale: 1.0
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" R A D A R N I X ".green());
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl App {
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up | KeyCode::Char('w') => self.nav.move_up(),
            KeyCode::Down | KeyCode::Char('s') => self.nav.move_down(),
            KeyCode::Left | KeyCode::Char('a') => self.nav.move_left(),
            KeyCode::Right | KeyCode::Char('d') => self.nav.move_right(),
            KeyCode::Char('+') => self.nav.zoom_in(),
            KeyCode::Char('-') => self.nav.zoom_out(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        todo!()
    }
}
