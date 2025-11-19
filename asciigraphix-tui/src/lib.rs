use std::{
    io,
    time::{Duration, Instant},
};

use asciigraphix_core::shapes::{Point, Point4, Shape, Shape4};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Margin, Rect}, prelude::CrosstermBackend, style::{Color, Style}, widgets::{Clear, Block, Borders, Gauge, Paragraph, Widget}, Frame, Terminal
};

use crate::{graphix::Graphix, header::Header, utils::ColorWrapper};

mod graphix;
mod header;
mod utils;

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
    help: bool,
    explore: bool,
    primary_color: ColorWrapper,
    fg_color: ColorWrapper,
    bg_color: ColorWrapper
}

impl Default for App {
    fn default() -> Self {
        const L: f64 = 30.0;
        App {
            shape: Shape::generate_cube(Point::zero(), 1.0),
            shape4: Shape4::generate_4d_paralellepiped(
                Point4::zero() - Point4(1.0, 1.0, 0.7, 0.7) * L / 2.0,
                Point4::e(1) * L,
                Point4::e(2) * L,
                Point4::e(3) * 0.7 * L,
                Point4::e(4) * 0.7 * L,
            ),
            cam_pos: Point(0.0, -80.0, 0.0),
            cam_direction: Point(0.0, 1.0, 0.0),
            rotations3d: (0.01, 0.0, 0.01),
            rotations4d: (0.0, 0.0, 0.0, 0.01, 0.00, 0.01),
            header_text: String::from("H"),
            header_cursor_blink_state: 1.0,
            last_time: Instant::now(),
            dt: Duration::from_millis(0),
            elapsed: Duration::from_millis(0),
            confusion: 0,
            reset: false,
            paused: false,
            exit: false,
            help: false,
            explore: false,
            primary_color: ColorWrapper::rgb(192, 80, 80),
            fg_color: ColorWrapper::rgb(240, 240, 240),
            bg_color: ColorWrapper::rgb(30, 30, 30)
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
    pub fn handle_event(&mut self, event: Event) {
        const ROTATION_AMOUNT: f64 = 0.01;
        const CAM_ROTATION: (f64, f64) = (0.04, 0.04);
        const CAM_SPEED: f64 = 0.5;
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => {
                self.exit = true;
            }
            Event::Key(k) => match k.code {
                KeyCode::Char('r') => {
                    self.reset = true;
                }
                KeyCode::Char(' ') => {
                    self.paused = !self.paused;
                }
                KeyCode::Char('?') => {
                    self.help = !self.help;
                    // self.paused = self.help;
                }
                KeyCode::Char('Q') => {
                    self.explore = !self.explore;
                }
                d if !self.explore => match d {
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
                    _ => {}
                },
                d if self.explore => match d {
                    KeyCode::Char('w') => {
                        self.cam_pos += self.cam_direction * CAM_SPEED;
                    }
                    KeyCode::Char('s') => {
                        self.cam_pos -= self.cam_direction * CAM_SPEED;
                    }
                    KeyCode::Char('a') => {
                        self.cam_pos -=
                            Point(self.cam_direction.1, -self.cam_direction.0, 0.0) * CAM_SPEED;
                    }
                    KeyCode::Char('d') => {
                        self.cam_pos +=
                            Point(self.cam_direction.1, -self.cam_direction.0, 0.0) * CAM_SPEED;
                    }
                    KeyCode::Up => {
                        self.cam_direction.rotate(0.0, CAM_ROTATION.1);
                    }
                    KeyCode::Down => {
                        self.cam_direction.rotate(0.0, -CAM_ROTATION.1);
                    }
                    KeyCode::Left => {
                        self.cam_direction.rotate(CAM_ROTATION.0, 0.0);
                    }
                    KeyCode::Right => {
                        self.cam_direction.rotate(-CAM_ROTATION.0, 0.0);
                    }
                    KeyCode::Char('h') => {
                        self.rotations4d.0 += ROTATION_AMOUNT;
                    }
                    KeyCode::Char('H') => {
                        self.rotations4d.0 -= ROTATION_AMOUNT;
                    }
                    KeyCode::Char('j') => {
                        self.rotations4d.1 += ROTATION_AMOUNT;
                    }
                    KeyCode::Char('J') => {
                        self.rotations4d.1 -= ROTATION_AMOUNT;
                    }
                    KeyCode::Char('k') => {
                        self.rotations4d.2 += ROTATION_AMOUNT;
                    }
                    KeyCode::Char('K') => {
                        self.rotations4d.2 -= ROTATION_AMOUNT;
                    }
                    KeyCode::Char('l') => {
                        self.rotations4d.3 += ROTATION_AMOUNT;
                    }
                    KeyCode::Char('L') => {
                        self.rotations4d.3 -= ROTATION_AMOUNT;
                    }
                    KeyCode::Char('n') => {
                        self.rotations4d.4 += ROTATION_AMOUNT;
                    }
                    KeyCode::Char('N') => {
                        self.rotations4d.4 -= ROTATION_AMOUNT;
                    }
                    KeyCode::Char('m') => {
                        self.rotations4d.5 += ROTATION_AMOUNT;
                    }
                    KeyCode::Char('M') => {
                        self.rotations4d.5 -= ROTATION_AMOUNT;
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(5))? {
            self.handle_event(event::read()?);
        };
        Ok(())
    }

    pub fn update(&mut self) -> io::Result<()> {
        if self.reset {
            self.reset = false;

            self.header_cursor_blink_state = 1.0;
            self.confusion = 0;

            self.header_text = String::from("H");

            self.cam_direction = Point(0.0, 1.0, 0.0);
            self.cam_pos = Point(0.0, -80.0, 0.0);

            const L: f64 = 30.0;
            self.shape4 = Shape4::generate_4d_paralellepiped(
                Point4::zero() - Point4(1.0, 1.0, 0.7, 0.7) * L / 2.0,
                Point4::e(1) * L,
                Point4::e(2) * L,
                Point4::e(3) * 0.7 * L,
                Point4::e(4) * 0.7 * L,
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

        if !self.explore {
            if self.confusion == 0 {
                self.rotations4d = (0.00, 0.00, 0.0, 0.00, 0.0, 0.0);
            } else if self.confusion <= 20 {
                self.rotations4d = (0.00, 0.00, 0.00, 0.01, 0.00, 0.0);
            } else if self.confusion <= 40 {
                self.rotations4d = (0.00, 0.00, 0.00, 0.00, 0.01, 0.0);
            } else if self.confusion <= 60 {
                self.rotations4d = (0.00, 0.00, 0.00, 0.00, 0.00, 0.01);
            } else if self.confusion <= 80 {
                self.rotations4d = (0.00, 0.0, 0.00, 0.01, 0.01, 0.00);
            } else if self.confusion <= 100 {
                self.rotations4d = (0.01, 0.00, 0.00, 0.01, 0.01, 0.01);
            }
        }

        self.shape4.rotate(&Point4::zero(), self.rotations4d);

        if self.header_text.len() >= String::from(HEADER).len() {
            self.header_cursor_blink_state =
                (2.0 * (-(0.004 * (self.elapsed.as_millis() as f64)).sin()).tanh() + 1.0) / 2.0;
        }

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
        // actual tesseract render
        Graphix::new(
            &self.shape4.project_to_3d(),
            self.cam_pos,
            self.cam_direction,
            self.primary_color.lighten(1.5),
            self.bg_color
        )
        .render(area, buf);

        if !self.explore {
            // header
            let default_color = 0.0;
            let cursor_style = Style::new().fg(Color::Rgb(
                (default_color + 155.0 * self.header_cursor_blink_state) as u8,
                (default_color + 155.0 * self.header_cursor_blink_state) as u8,
                (default_color + 155.0 * self.header_cursor_blink_state) as u8,
            ));
            let header_style = Style::new().fg(self.primary_color.into());

            // confusion meter
            let h = Header::new(self.header_text.clone(), header_style, cursor_style);
            let h_height = (&h.height).clone() as u16;
            let h_width = (&h.width).clone() as u16;
            h.render(
                Rect::new(area.width / 2 - h_width / 2, 5, h_width, h_height),
                buf,
            );
            Gauge::default()
                .gauge_style(Style::new().fg(self.primary_color.into()).bg(self.bg_color.into()))
                .percent(self.confusion)
                .style(Style::new().fg(self.primary_color.into()).bg(self.bg_color.into()))
                .block(
                    Block::new()
                        .borders(Borders::TOP | Borders::BOTTOM)
                        .title(" Confusion ")
                        .title_alignment(Alignment::Center)
                        .title_style(Style::new().fg(self.primary_color.into()))
                        .border_style(Style::new().fg(self.primary_color.into()).bg(self.bg_color.into()))
                        .title_bottom(" w increase | s decrease | space pause | r reset | ? info "),
                )
                .render(Rect::new(area.width / 2 - 35, area.height - 6, 75, 5), buf);
        } else {
            let ar = Rect::new(area.width / 2 - 20, area.height - 6, 40, 2);
            Clear.render(ar, buf);
            Paragraph::new(format!(
                "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2}",
                self.rotations4d.0,
                self.rotations4d.1,
                self.rotations4d.2,
                self.rotations4d.3,
                self.rotations4d.4,
                self.rotations4d.5
            ))
            .block(Block::new().title(" Rotations ").title_alignment(Alignment::Center))
            .style(Style::new().fg(self.fg_color.into()).bg(self.bg_color.into())).alignment(Alignment::Center)
            .render(ar, buf);
        }

        // help popup
        if self.help {
            let help_area = area.inner(Margin::new(40, 15));
            Clear.render(help_area, buf);
            Paragraph::new(
                "In four dimensions you can rotate an object along six different _planes_.

In contrast, in three dimensions you have three _lines_ (axes).

\"Confusion\" adds/changes what rotations are being applied.

For example: 
    20% rotates along only one plane,
    60% rotates along two different planes,
    ...


Additionally, by pressing Q you can enter \"explore\"-mode.
In this mode you can use:
    - w,a,s,d to move,
    - arrow keys to look,
    - h,j,k,l,n,m to inrease rotation on a plane of rotation,
    - H,J,K,L,N,M to decrease rotation on a plane of rotation
                ",
            )
            .block(
                Block::bordered()
                    .title(" What is going on here? ")
                    .title_alignment(Alignment::Center),
            )
            .style(Style::new().fg(self.fg_color.into()).bg(self.bg_color.into()))
            .render(help_area, buf);
        }
    }
}
