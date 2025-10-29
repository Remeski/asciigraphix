use asciigraphix_core::{
    shapes::{Point, Shape}, Display
};

fn main() {
    const DISPLAY_X: usize = 100;
    const DISPLAY_Y: usize = 50;
    let mut display = Display::new(
        DISPLAY_X,
        DISPLAY_Y,
        Point(0.0, -10.0, -30.0),
        Point(0.0, 0.0, 1.0),
        20.0,
    );
    let mut cube = Shape::generate_cube(Point(0.0, 0.0, 0.0), 14.0);
    loop {
        cube.rotate(
            &cube.center.clone().unwrap_or(Point(0.0, 0.0, 0.0)),
            (0.05, 0.05, 0.0),
        );
        display.render_print(&cube);

        std::thread::sleep(std::time::Duration::from_millis(33));
    }
}
