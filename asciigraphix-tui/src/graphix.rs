use asciigraphix_core::{shapes::{Point, Shape}, Display};
use ratatui::{style::{Color, Style, Stylize}, widgets::Widget};

use crate::utils::ColorWrapper;

pub struct Graphix<'a> {
    shape: &'a Shape,
    cam_pos: Point,
    cam_direction: Point,
    accent_color: ColorWrapper,
    bg_color: ColorWrapper

}

impl<'a> Graphix<'a> {
    pub fn new(shape: &'a Shape, cam_pos: Point, cam_direction: Point, accent_color: ColorWrapper, bg_color: ColorWrapper) -> Self {
        Self {
            shape,
            cam_pos,
            cam_direction,
            accent_color,
            bg_color
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

            // linear interpolation from 0.5 - 1.0 as depth 30.0 - 100.0
            let coef = (-0.5/70.0 * (depth - 30.0) + 1.0).min(1.0).max(0.5);
            let r = coef * self.accent_color.r as f32;
            let g = coef * self.accent_color.g as f32;
            let b = coef * self.accent_color.b as f32;
            let mut color = Color::Rgb(r as u8, g as u8, b as u8);
            if *depth == 0.0 {
                color = Color::Rgb(0,0,0);
            }
            let mut style = Style::new()
                .fg(color)
                .bg(self.bg_color.into());
            if *depth < 60.0 {
                style = style.bold();
            }
            buf.set_string(area.x + x as u16, area.y + y as u16, str, style);
        }
    }
}
