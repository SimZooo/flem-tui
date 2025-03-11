use std::io::{self, Error};
mod gap_buffer;
use crossterm::event::{self, Event, KeyEventKind, KeyCode};
use gap_buffer::*;
use ratatui::{DefaultTerminal, Frame, widgets::{Widget, Paragraph}, layout::{Position, Layout, Direction, Constraint}};

#[derive(Debug)]
pub struct App {
    pub exit: bool,
    pub buffer: GapBuffer<char>,
    // (Line, n)
    pub lines: Vec<(usize, usize)>,
}

impl App {
    fn from_file(file: &String) -> Result<Self, ()> {
        if let Ok(content) = std::fs::read(file) {
            let gap_buffer = GapBuffer::from(content);
            Ok(App {
                exit: false,
                buffer: gap_buffer,
                lines: Vec::default()
            })
        } else {
            Err(())
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            exit: false,
            buffer: GapBuffer::new(10, '\0'),
            lines: vec![(1, 0); 1]
        }
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let layout = Layout::default().direction(Direction::Vertical).constraints(
            vec![Constraint::Percentage(95), Constraint::Percentage(5)]
            ).split(area);
        let info_layout = Layout::default().direction(Direction::Horizontal).constraints(
            vec![Constraint::Percentage(80), Constraint::Percentage(20)]
            ).split(area);
        Paragraph::new(self.buffer.to_string()).render(layout[0], buf);
        Paragraph::new(self.get_cursor_pos().to_string()).render(layout[1], buf);
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
        frame.set_cursor_position(self.get_cursor_pos());
    }

    fn get_cursor_pos(&self) -> Position {
        let char_info = self.lines.last();
        if let Some(char_info) = char_info {
            if let Some(last) = self.lines.last() {
                let cursor_x = self.buffer.len() as i32 - self.buffer.cursor as i32 + last.1 as i32;
                return Position::new(cursor_x as u16, char_info.0 as u16 - 1);
            }
        }
        Position::default()
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char(ch) => {
                        self.buffer.insert(ch);
                        if let Some(last) = self.lines.last_mut() {
                            last.1 += 1;
                        }
                    },
                    KeyCode::Esc => {
                        self.exit = true;
                    },
                    KeyCode::Backspace => {
                        self.buffer.delete();
                        if let Some(last) = self.lines.last_mut() {
                            if last.1 > 0 {
                                last.1 -= 1;
                            } else if self.lines.len() > 1 {
                                self.lines.pop();
                            }
                        }
                    },
                    KeyCode::Enter => {
                        self.buffer.insert('\n');
                        let n = self.lines.len();
                        self.lines.push((n + 1, 0));
                    },
                    KeyCode::Left => {
                        self.buffer.left();
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
    let args = std::env::args();
    let app_result;

    if args.len() == 2 {
        let args: Vec<String> = args.collect();
        let filename = &args[1];
        let app = App::from_file(filename);
        if let Ok(mut app) = app {
            app_result = app.run(&mut terminal);
        } else {
            app_result = Err(io::Error::new(io::ErrorKind::NotFound, format!("Failed to find file: {}", filename)));
        }
    } else {
        app_result = App::default().run(&mut terminal);
    }

    ratatui::restore();
    app_result
}
