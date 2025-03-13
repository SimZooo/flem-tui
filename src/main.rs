use std::io::{self, Error};
mod gap_buffer;
use crossterm::event::{self, Event, KeyEventKind, KeyCode};
use gap_buffer::*;
use ratatui::{DefaultTerminal, Frame, widgets::{Widget, Paragraph}, layout::{Position, Layout, Direction, Constraint, Positions}};

#[derive(Debug)]
pub struct App {
    pub exit: bool,
    pub buffer: GapBuffer<char>,
    // (Line, n)
    pub lines: Vec<usize>,
    pub filename: String,
}

impl App {
    fn from_file(file: &String) -> Result<Self, ()> {
        if let Ok(content) = std::fs::read(file) {
            let gap_buffer = GapBuffer::from(content);
            let mut lines = Vec::default();
            for c in &gap_buffer.buffer {
                if *c == '\n' {
                    lines.push(0);
                } else {
                    if let Some(last) = lines.last_mut() {
                        *last += 1;
                    }
                }
            }
            Ok(App {
                exit: false,
                buffer: gap_buffer,
                lines,
                filename: file.to_string()
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
            lines: vec![0],
            filename: "undefined".to_string()
        }
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let layout = Layout::default().direction(Direction::Vertical).constraints(
            vec![Constraint::Percentage(95), Constraint::Percentage(5)]
            ).split(area);
        let info_layout = Layout::default().direction(Direction::Horizontal).constraints(
            vec![Constraint::Percentage(96), Constraint::Percentage(4)]
            ).split(layout[1]);
        Paragraph::new(self.buffer.to_string()).render(layout[0], buf);

        let cursor_pos = self.get_cursor_pos();
        Paragraph::new(self.filename.clone()).render(info_layout[0], buf);
        Paragraph::new(format!("{}:{}", cursor_pos.x, cursor_pos.y)).render(info_layout[1], buf);
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
        let mut position = 0;
        for (i, line) in self.lines.iter().enumerate() {
            if self.buffer.cursor < position + line {
                return Position::new(self.buffer.cursor as u16  - position as u16, i as u16);
            }
            position += *line;
        }
        return Position::new(self.lines[self.lines.len() - 1] as u16, self.lines.len() as u16 - 1);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char(ch) => {
                        self.buffer.insert(ch);
                        if let Some(last) = self.lines.last_mut() {
                            *last += 1;
                        }
                    },
                    KeyCode::Esc => {
                        self.exit = true;
                    },
                    KeyCode::Backspace => {
                        self.buffer.delete();
                        if let Some(last) = self.lines.last_mut() {
                            if *last > 0 {
                                *last -= 1;
                            } else if self.lines.len() > 1 {
                                self.lines.pop();
                            }
                        }
                    },
                    KeyCode::Enter => {
                        self.buffer.insert('\n');
                        self.lines.push(0);
                    },
                    KeyCode::Left => {
                        self.buffer.left();
                    },
                    KeyCode::Right => {
                        self.buffer.right();
                    },
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
