use asciigraphix_core::{Display, math::{Point, Point4}, shapes::Shape4};

fn main() {
    const DISPLAY_X: usize = 100;
    const DISPLAY_Y: usize = 50;
    let mut display = Display::new(
        DISPLAY_X,
        DISPLAY_Y,
        Point(0.0, 0.0, 20.0),
        Point(0.0, 0.0, -1.0),
        Point(0.0, 1.0, 0.0),
        3.14 / 4.0,
    );
    let l = 2.0;
    let center = Point4(-5.0, -5.0, -5.0, -5.0) * l;
    let mut tesseract = Shape4::generate_4d_paralellepiped(
        center,
        Point4(10.0, 0.0, 0.0, 0.0) * l,
        Point4(0.0, 10.0, 0.0, 0.0) * l,
        Point4(0.0, 0.0, 10.0, 0.0) * l,
        Point4(0.0, 0.0, 0.0, 10.0) * l,
    );

    Display::clear_screen();
    loop {
        tesseract.rotate(
            &Point4(0.0, 0.0, 0.0, 0.0),
            (0.0, 0.0, 0.0, 0.02, 0.00, 0.00),
        );

        display.render_terminal(&tesseract.project_to_3d());

        std::thread::sleep(std::time::Duration::from_millis(33));
    }
}
