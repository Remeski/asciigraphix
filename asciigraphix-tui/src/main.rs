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
    // let mut cube = Shape::generate_cube(Point(0.0, 0.0, 0.0), 20.0);
    let l = 4.0;
    let start = Point4(-5.0,-5.0,-5.0,-5.0)*l;
    let mut tesseract = Shape4::generate_4d_paralellepiped(start, Point4(10.0, 0.0, 0.0,0.0)*l, Point4(0.0,10.0,0.0,0.0)*l, Point4(0.0,0.0,10.0,0.0)*l, Point4(0.0,0.0,0.0,10.0)*l);

    let mut speed_1 = 0.0;
    let mut speed_2 = 0.0;
    let mut speed_3 = 0.0;
    let mut speed_4 = 0.0;
    let mut speed_5 = 0.0;
    let mut speed_6 = 0.0;

    loop {
        tesseract.rotate(
            &Point4(0.0, 0.0, 0.0, 0.0),
            (speed_1 * 0.1, speed_2 * 0.1, speed_3 * 0.1, speed_4 * 0.1, speed_5 * 0.1, speed_6 * 0.1),
        );

        terminal.draw(|frame | render(frame, &mut tesseract.project_to_3d(), (speed_1, speed_2, speed_3, speed_4, speed_5, speed_6)))?;

        if event::poll(Duration::from_millis(10))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Char('q') => {
                            break Ok(())
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

fn render(frame: &mut Frame, shape: &mut Shape, rots: (f64, f64, f64, f64, f64, f64)) {
    let t_area = Rect::new(0, 0, frame.area().width, 1);
    frame.render_widget(Paragraph::new(format!("{:.1} {:.1} {:.1} {:.1} {:.1} {:.1}", rots.0, rots.1, rots.2, rots.3, rots.4, rots.5)), t_area);
    frame.render_stateful_widget(Graphix::new(), Rect::new(0,1,frame.area().width, frame.area().height - 1), shape);
}

struct Graphix;

impl Graphix {
    pub fn new() -> Self {
        Self {}
    }
}

impl StatefulWidget for Graphix {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let mut display = Display::new(
            area.width as usize,
            area.height as usize,
            Point(0.0, 0.0, -30.0),
            Point(0.0, 0.0, 1.0),
            10.0,
        );
        for (i,line) in display.render(state).lines().enumerate() {
            buf.set_string(area.x, (area.y as usize + i) as u16, line, Style::default());
        }
    }

    type State = Shape;
}
