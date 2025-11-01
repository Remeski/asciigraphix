use asciigraphix_core::{shapes::{Point, Shape}, Display, RGB};
use ratatui::{style::{Color, Style}, widgets::Widget};

pub struct Graphix<'a> {
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
            100.0,
        );

        for (i, (_, color)) in display.render(&self.shape).iter().enumerate() {
            let x = i % area.width as usize;
            let y = (i - x) / area.width as usize;

            let str = "∷";

            buf.set_string(area.x + x as u16, area.y + y as u16, str, Style::new().fg(Color::from_u32(*color)));
        }
    }
}
