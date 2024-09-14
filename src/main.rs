use std::io::{self, Stdout};
use crossterm::event::{Event, KeyEvent};
use ratatui::{
    buffer::Buffer, crossterm::event::{self, KeyCode, KeyEventKind}, layout::{Alignment, Rect}, style::{Color, Stylize}, symbols::{border, Marker}, text::{Line, Text}, widgets::{
        block::{Position, Title}, canvas::{Canvas, Circle, Map, MapResolution, Rectangle}, Block, Borders, Padding, Paragraph, Widget}, DefaultTerminal, Frame
};

mod models;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}

struct App {
    marker: Marker,
}

impl Default for App {
    fn default() -> Self {
        App {
            marker: Marker::Braille
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from("  ✈ R A D A R N I X ✈  ".green());
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .border_set(border::THICK)
            .green();

        block.render(area, buf)
    }
}

impl App {
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => { return Ok(()); },
                        _ => {}
                    }
                }
            }
        }
    }

    fn map_canvas(&self) -> impl Widget + '_ {
        let block = Block::new().borders(Borders::empty()).padding(Padding::uniform(1));
        Canvas::default()
            .marker(self.marker)
            .block(block)
            .paint(|ctx| {
                ctx.draw(&Map {
                    color: Color::Green,
                    resolution: MapResolution::High
                });
            })
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0])
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
        frame.render_widget(self.map_canvas(), frame.area());
    }

}
