use std::{io, time::{Duration, Instant}};

use asciigraphix_core::shapes::{Point, Point4, Shape, Shape4};
use crossterm::{event::{self, Event, KeyCode, KeyEvent}};
use ratatui::{
    layout::Rect, style::{Color, Style}, widgets::{Widget}, DefaultTerminal, Frame
};

use crate::{graphix::Graphix, header::Header};

mod graphix;
mod header;

pub struct App {
    shape: Shape,
    shape4: Shape4,
    cam_pos: Point,
    cam_direction: Point,
    rotations3d: (f64, f64, f64),
    rotations4d: (f64, f64, f64, f64, f64, f64),
    header_cursor_blink_state: f64,
    header_text: String,
    last_time: Instant,
    dt: Duration,
    elapsed: Duration,
    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        const L: f64 = 2.5;
        App {
            shape: Shape::generate_cube(Point::zero(), 1.0),
            shape4: Shape4::generate_4d_paralellepiped(Point4::zero() - Point4(1.0, 1.0, 1.0, 1.0) * L/2.0, Point4::e(1) * L, Point4::e(2) * L, Point4::e(3) * L, Point4::e(4) * L),
            cam_pos: Point(0.0, -10.0, 0.0),
            cam_direction: Point(0.0, 1.0, 0.0),
            rotations3d: (0.01, 0.0, 0.01),
            rotations4d: (0.0, 0.0, 0.0, 0.01, 0.00, 0.01),
            header_cursor_blink_state: 0.0,
            last_time: Instant::now(),
            dt: Duration::from_millis(0),
            elapsed: Duration::from_millis(0),
            header_text: String::from("H"),
            exit: false,
        }
    }
}

const HEADER: &str = "TeSSHeract";

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            self.dt = Instant::now() - self.last_time;
            self.elapsed += self.dt;
            self.last_time = Instant::now();
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
                            self.rotations3d.0 += 0.01;
                        }
                        KeyCode::Char('H') => {
                            self.rotations3d.0 -= 0.01;
                        }
                        KeyCode::Char('j') => {
                            self.rotations3d.1 += 0.01;
                        }
                        KeyCode::Char('J') => {
                            self.rotations3d.1 -= 0.01;
                        }
                        KeyCode::Char('l') => {
                            self.rotations3d.2 += 0.01;
                        }
                        KeyCode::Char('L') => {
                            self.rotations3d.2 -= 0.01;
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
        if self.elapsed.as_secs_f64() > 0.4 && self.elapsed.as_millis() % 100 <= 25 {
            self.header_text = String::from(HEADER).chars().take(self.header_text.len() + 1).collect();
        }

        self.shape.rotate(&self.shape.center.unwrap_or(Point::zero()), self.rotations3d);
        self.shape4.rotate(&Point4::zero(), self.rotations4d);

        self.header_cursor_blink_state = (2.0*(-(0.004*(self.elapsed.as_millis() as f64)).sin()).tanh() + 1.0) / 2.0;
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
        Graphix::new(&self.shape4.project_to_3d(), self.cam_pos, self.cam_direction).render(area, buf);

        let default_color = 0.0;
        let cursor_style = Style::new().fg(Color::Rgb((default_color + 155.0 * self.header_cursor_blink_state) as u8, (default_color + 155.0 * self.header_cursor_blink_state) as u8, (default_color + 155.0 * self.header_cursor_blink_state) as u8));
        let header_style = Style::new().fg(ratatui::style::Color::Red);

        let h = Header::new(self.header_text.clone(), header_style, cursor_style);
        let h_height = (&h.height).clone() as u16;
        let h_width = (&h.width).clone() as u16;
        h.render(Rect::new(area.width / 2 - h_width / 2, 5, h_width, h_height), buf);
    }
}
