use figlet_rs::FIGfont;
use ratatui::{style::Style, widgets::Widget};

pub struct Header {
    text: String,
    style: Style,
    pub height: usize,
    pub width: usize,
}

impl Header {
    pub fn new(text: String, style: Style) -> Self {
        let text_rendered = Self::render(text);
        let height = (&text_rendered).lines().count();
        let width = (&text_rendered).lines().next().unwrap_or("").len();

        Self {
            text: text_rendered,
            style,
            height, 
            width 
        }
    }

    fn render(text: String) -> String {
        let standard = FIGfont::standard().unwrap();
        let figure = standard.convert(&text);
        figure.unwrap().to_string()
    }
}

impl Widget for Header {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        for (i, line) in self.text.lines().enumerate() {
            buf.set_string(area.x, area.y + i as u16, line, self.style);
        }
    }
}
