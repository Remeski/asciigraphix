use ratatui::style::Color;

#[derive(Clone, Copy)]
pub struct ColorWrapper {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorWrapper {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// factor 1.0 changes nothing. factor > 1.0 will brighten.
    pub fn lighten(&self, factor: f32) -> Self {
        let factor = factor;
        let r = factor * self.r as f32;
        let g = factor * self.g as f32;
        let b = factor * self.b as f32;
        let r = r.min(254.999) as u8;
        let g = g.min(254.999) as u8;
        let b = b.min(254.999) as u8;
        Self { r, g, b }
    }
}
impl From<ColorWrapper> for Color {
    fn from(value: ColorWrapper) -> Self {
        Self::Rgb(value.r, value.g, value.b)
    }
}
