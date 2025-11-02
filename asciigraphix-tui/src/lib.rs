use std::{
    io,
    time::{Duration, Instant},
};

use asciigraphix_core::shapes::{Point, Point4, Shape, Shape4};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Rect}, prelude::CrosstermBackend, style::{Color, Style, Stylize}, widgets::{Block, Borders, Gauge, Widget}, Frame, Terminal
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
    confusion: u16,
    reset: bool,
    paused: bool,
    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        const L: f64 = 2.5;
        App {
            shape: Shape::generate_cube(Point::zero(), 1.0),
            shape4: Shape4::generate_4d_paralellepiped(
                Point4::zero() - Point4(1.0, 1.0, 1.0, 1.0) * L / 2.0,
                Point4::e(1) * L,
                Point4::e(2) * L,
                Point4::e(3) * L,
                Point4::e(4) * L,
            ),
            cam_pos: Point(0.0, -10.0, 0.0),
            cam_direction: Point(0.0, 1.0, 0.0),
            rotations3d: (0.01, 0.0, 0.01),
            rotations4d: (0.0, 0.0, 0.0, 0.01, 0.00, 0.01),
            header_cursor_blink_state: 0.0,
            last_time: Instant::now(),
            dt: Duration::from_millis(0),
            elapsed: Duration::from_millis(0),
            header_text: String::from("H"),
            confusion: 0,
            reset: false,
            paused: false,
            exit: false,
        }
    }
}

const HEADER: &str = "TeSSHeract";

impl App {
    pub fn run<T: std::io::Write>(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<T>>,
    ) -> io::Result<()> {
        while !self.exit {
            self.handle_events()?;
            self.update()?;
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(5))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => {
                    self.exit = true;
                }
                Event::Key(k) => match k.code {
                    KeyCode::Up => {
                        self.cam_pos += self.cam_direction * 0.2;
                    }
                    KeyCode::Down => {
                        self.cam_pos -= self.cam_direction * 0.2;
                    }
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
                    KeyCode::Char('w') => {
                        if self.confusion <= 80 {
                            self.confusion += 20
                        }
                    }
                    KeyCode::Char('s') => {
                        if self.confusion >= 20 {
                            self.confusion -= 20
                        }
                    }
                    KeyCode::Char('r') => {
                        self.reset = true;
                    }
                    KeyCode::Char(' ') => {
                        self.paused = !self.paused;
                    }
                    _ => {}
                },
                _ => {}
            }
        };
        Ok(())
    }

    pub fn update(&mut self) -> io::Result<()> {
        if self.reset {
            self.reset = false;

            self.header_text = String::from("H");
            const L: f64 = 2.5;
            self.shape4 = Shape4::generate_4d_paralellepiped(
                Point4::zero() - Point4(1.0, 1.0, 1.0, 1.0) * L / 2.0,
                Point4::e(1) * L,
                Point4::e(2) * L,
                Point4::e(3) * L,
                Point4::e(4) * L,
            );
        }

        if self.paused {
            return Ok(());
        }

        self.dt = Instant::now() - self.last_time;
        self.elapsed += self.dt;
        self.last_time = Instant::now();

        if self.elapsed.as_secs_f64() > 0.4 && self.elapsed.as_millis() % 100 <= 25 {
            self.header_text = String::from(HEADER)
                .chars()
                .take(self.header_text.len() + 1)
                .collect();
        }

        self.shape.rotate(
            &self.shape.center.unwrap_or(Point::zero()),
            self.rotations3d,
        );

        if self.confusion == 0 {
            self.rotations4d = (0.01, 0.00, 0.0, 0.0, 0.0, 0.0);
        } else if self.confusion <= 20 {
            self.rotations4d = (0.00, 0.01, 0.00, 0.00, 0.00, 0.0);
        } else if self.confusion <= 40 {
            self.rotations4d = (0.00, 0.00, 0.00, 0.01, 0.01, 0.0);
        } else if self.confusion <= 60 {
            self.rotations4d = (0.00, 0.01, 0.00, 0.01, 0.00, 0.00);
        } else if self.confusion <= 80 {
            self.rotations4d = (0.01, 0.0, 0.00, 0.01, 0.00, 0.00);
        } else if self.confusion <= 100 {
            self.rotations4d = (0.01, 0.00, 0.00, 0.01, 0.01, 0.00);
        }

        self.shape4.rotate(&Point4::zero(), self.rotations4d);

        self.header_cursor_blink_state =
            (2.0 * (-(0.004 * (self.elapsed.as_millis() as f64)).sin()).tanh() + 1.0) / 2.0;
        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Graphix::new(
            &self.shape4.project_to_3d(),
            self.cam_pos,
            self.cam_direction,
        )
        .render(area, buf);

        let default_color = 0.0;
        let cursor_style = Style::new().fg(Color::Rgb(
            (default_color + 155.0 * self.header_cursor_blink_state) as u8,
            (default_color + 155.0 * self.header_cursor_blink_state) as u8,
            (default_color + 155.0 * self.header_cursor_blink_state) as u8,
        ));
        let header_style = Style::new().fg(ratatui::style::Color::Red);

        let h = Header::new(self.header_text.clone(), header_style, cursor_style);
        let h_height = (&h.height).clone() as u16;
        let h_width = (&h.width).clone() as u16;
        h.render(
            Rect::new(area.width / 2 - h_width / 2, 5, h_width, h_height),
            buf,
        );

        Gauge::default()
            .gauge_style(Style::new().fg(Color::Red).bg(Color::Reset))
            .percent(self.confusion)
            .style(Style::new().fg(Color::White))
            .block(
                Block::new()
                    .borders(Borders::TOP | Borders::BOTTOM)
                    .title(" Confusion ")
                    .title_alignment(Alignment::Center)
                    .title_style(Style::new().fg(Color::Red))
                    .border_style(Style::new().red())
                    .title_bottom(" w increase | s decrease | space pause | r reset "),
            )
            .render(Rect::new(area.width / 2 - 35, area.height - 6, 75, 5), buf);
    }
}
