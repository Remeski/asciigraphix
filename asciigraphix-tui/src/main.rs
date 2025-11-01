use std::io;

use ratatui;
use asciigraphix_tui::App;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::default();
    let result = app.run(&mut terminal);
    ratatui::restore();
    result
}
