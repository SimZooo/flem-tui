use std::io;
mod gap_buffer;
use crossterm::event::{self, Event, KeyEventKind, KeyCode};
use gap_buffer::*;
use ratatui::{DefaultTerminal, Frame, widgets::{Widget, Paragraph}, layout::Position};

#[derive(Debug)]
pub struct App {
    pub exit: bool,
    pub buffer: GapBuffer<char>
}

impl Default for App {
    fn default() -> Self {
        Self {
            exit: false,
            buffer: GapBuffer::new(10, '\0')
        }
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Paragraph::new(self.buffer.to_string()).render(area, buf);
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| {
                self.draw(frame)
            })?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
        frame.set_cursor_position(Position::new(self.buffer.cursor as u16, 0))
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char(ch) => {
                        self.buffer.insert(ch);
                    },
                    KeyCode::Esc => {
                        self.exit = true;
                    },
                    KeyCode::Backspace => {
                        self.buffer.delete();
                    }
                    _ => {}
                }
            },
            _ => {}
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);

    ratatui::restore();
    app_result
}
