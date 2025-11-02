use asciigraphix_core::{shapes::{Point, Shape}, Display};
use ratatui::{style::{Color, Style, Stylize}, widgets::Widget};

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

        for (i, (depth, _)) in display.render(&self.shape).iter().enumerate() {
            let x = i % area.width as usize;
            let y = (i - x) / area.width as usize;

            let str = "∷";

            // linear from 0.5 - 1.0 as depth 5.0 - 10.0
            let coef = (-0.5/8.0 * (depth - 5.0) + 1.0).min(1.0).max(0.5);
            let c = (255.0 * coef) as u8;
            let mut color = Color::Rgb(c, 0, 0);
            if *depth == 0.0 {
                color = Color::Rgb(0,0,0);
            }
            let mut style = Style::new()
                .fg(color);
            if *depth < 5.0 {
                style = style.bold();
            }
            buf.set_string(area.x + x as u16, area.y + y as u16, str, style);
        }
    }
}
