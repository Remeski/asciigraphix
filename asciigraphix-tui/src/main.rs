use std::{io, time::Duration};

use asciigraphix_core::shapes::{Point, Shape};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    layout::Rect, style::Style, widgets::{Paragraph, Widget}, DefaultTerminal, Frame
};

use crate::{graphix::Graphix, header::Header};

mod graphix;
mod header;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::default();
    let result = app.run(&mut terminal);
    ratatui::restore();
    result
}

struct App {
    shape: Shape,
    cam_pos: Point,
    cam_direction: Point,
    rotations: (f64, f64, f64),
    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        App {
            shape: Shape::generate_cube(Point::zero(), 1.0),
            cam_pos: Point(0.0, -10.0, 0.0),
            cam_direction: Point(0.0, 1.0, 0.0),
            rotations: (0.0, 0.0, 0.0),
            exit: false,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            self.handle_events()?;
            self.update()?;
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(5))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => {
                    self.exit = true;
                }
                Event::Key(k) => {
                    match k.code {
                        KeyCode::Char('h') => {
                            self.rotations.0 += 0.01;
                        }
                        KeyCode::Char('H') => {
                            self.rotations.0 -= 0.01;
                        }
                        KeyCode::Char('j') => {
                            self.rotations.1 += 0.01;
                        }
                        KeyCode::Char('J') => {
                            self.rotations.1 -= 0.01;
                        }
                        KeyCode::Char('l') => {
                            self.rotations.2 += 0.01;
                        }
                        KeyCode::Char('L') => {
                            self.rotations.2 -= 0.01;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        };
        Ok(())
    }

    fn update(&mut self) -> io::Result<()> {
        self.shape.rotate(&self.shape.center.unwrap_or(Point::zero()), self.rotations);
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Graphix::new(&self.shape, self.cam_pos, self.cam_direction).render(area, buf);
        // Paragraph::new("Asciigraphix").render(Rect::new(area.width / 2 - "Asciigraphix".len() as u16 / 2, 3, "Asciigraphix".len() as u16, 1), buf);
        let h = Header::new(String::from("Asciigraphix"), Style::new().fg(ratatui::style::Color::Red));
        let h_height = (&h.height).clone() as u16;
        let h_width = (&h.width).clone() as u16;
        h.render(Rect::new(area.width / 2 - h_width / 2, 5, h_width, h_height), buf);
    }
}
