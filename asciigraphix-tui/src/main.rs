use std::{error::Error, time::Duration};

use asciigraphix_core::{shapes::{Point, Point4, Shape, Shape4}, Display};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, ModifierKeyCode};
use ratatui::{layout::Rect, style::Style, text::Line, widgets::{Paragraph, StatefulWidget, Widget}, DefaultTerminal, Frame};

fn main() -> Result<(), Box<dyn Error>> {
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<(), Box<dyn Error>> {
    let mut cube = Shape::generate_cube(Point(0.0, 0.0, 0.0), 20.0);
    // let p = Shape::generate_parallelepiped(Point(0.0, 0.0, 0.0), Point(10.0, 0.0, 0.0), Point(0.0, 10.0, 0.0), Point(0.0, 0.0, 10.0));
    // let l = 3.0;
    // let start = Point4(-5.0,-5.0,-5.0,-5.0)*l;
    // let mut tesseract = Shape4::generate_4d_paralellepiped(start, Point4(10.0, 0.0, 0.0,0.0)*l, Point4(0.0,10.0,0.0,0.0)*l, Point4(0.0,0.0,10.0,0.0)*l, Point4(0.0,0.0,0.0,10.0)*l);

    let mut speed_1 = 0.0;
    let mut speed_2 = 0.0;
    let mut speed_3 = 0.0;
    let mut speed_4 = 0.0;
    let mut speed_5 = 0.0;
    let mut speed_6 = 0.0;

    let mut cam_pos = Point(0.0, -30.0, 0.0);
    let mut cam_direction = Point(0.0, 1.0, 0.0);

    loop {
        // tesseract.rotate(
        //     &Point4(0.0, 0.0, 0.0, 0.0),
        //     (speed_1 * 0.1, speed_2 * 0.1, speed_3 * 0.1, speed_4 * 0.1, speed_5 * 0.1, speed_6 * 0.1),
        // );

        // let gx = Graphix::new(tesseract.project_to_3d(), cam_pos, cam_direction);
        let gx = Graphix::new(&cube, cam_pos, cam_direction);

        terminal.draw(|frame | render(frame, &gx, (speed_1, speed_2, speed_3, speed_4, speed_5, speed_6)))?;

        if event::poll(Duration::from_millis(10))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Char('q') => {
                            break Ok(())
                        }
                        KeyCode::Char('w') => {
                            cam_pos += cam_direction * 0.2;
                        }
                        KeyCode::Char('s') => {
                            cam_pos -= cam_direction * 0.2
                        }
                        KeyCode::Char('a') => {
                            cam_pos -= Point(cam_direction.1, -cam_direction.0, 0.0) * 0.2;
                        }
                        KeyCode::Char('d') => {
                            cam_pos += Point(cam_direction.1, -cam_direction.0, 0.0) * 0.2;
                        }
                        KeyCode::Up => {
                            cam_direction.rotate(0.0, 0.01);
                        }
                        KeyCode::Down => {
                            cam_direction.rotate(0.0, -0.01);
                        }
                        KeyCode::Left => {
                            cam_direction.rotate(0.01, 0.0);
                        }
                        KeyCode::Right => {
                            cam_direction.rotate(-0.01, 0.0);
                        }
                        KeyCode::Char('h') => {
                            speed_1 += 0.1;
                        }
                        KeyCode::Char('j') => {
                            speed_2 += 0.1;
                        }
                        KeyCode::Char('k') => {
                            speed_3 += 0.1; 
                        }
                        KeyCode::Char('l') => {
                            speed_4 += 0.1; 
                        }
                        KeyCode::Char('n') => {
                            speed_5 += 0.1; 
                        }
                        KeyCode::Char('m') => {
                            speed_6 += 0.1; 
                        }
                        KeyCode::Char('H') => {
                            speed_1 -= 0.1;
                        }
                        KeyCode::Char('J') => {
                            speed_2 -= 0.1;
                        }
                        KeyCode::Char('K') => {
                            speed_3 -= 0.1; 
                        }
                        KeyCode::Char('L') => {
                            speed_4 -= 0.1;
                        }
                        KeyCode::Char('N') => {
                            speed_5 -= 0.1; 
                        }
                        KeyCode::Char('M') => {
                            speed_6 -= 0.1; 
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        };
    }
}

fn render(frame: &mut Frame, gx: &Graphix, rots: (f64, f64, f64, f64, f64, f64)) {
    let t_area = Rect::new(0, 0, frame.area().width, 1);
    frame.render_widget(Paragraph::new(format!("{:.1} {:.1} {:.1} {:.1} {:.1} {:.1}", rots.0, rots.1, rots.2, rots.3, rots.4, rots.5)), t_area);
    frame.render_widget(gx, Rect::new(0,1,frame.area().width, frame.area().height - 1));
}

struct Graphix<'a> {
    shape: &'a Shape,
    cam_pos: Point,
    cam_direction: Point
}

impl<'a> Graphix<'a> {
    pub fn new(shape: &'a Shape, cam_pos: Point, cam_direction: Point) -> Self {
        Self {
            shape,
            cam_pos,
            cam_direction
        }
    }
}

impl<'a> Widget for &Graphix<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mut display = Display::new(
            area.width as usize,
            area.height as usize,
            self.cam_pos,
            self.cam_direction,
            10.0,
        );
        for (i,line) in display.render(&self.shape).lines().enumerate() {
            buf.set_string(area.x, (area.y as usize + i) as u16, line, Style::default());
        }
    }
}
