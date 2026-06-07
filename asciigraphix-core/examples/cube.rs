use asciigraphix_core::{
    Display,
    shapes::{Face, Point, Shape},
};

fn main() {
    const DISPLAY_X: usize = 100;
    const DISPLAY_Y: usize = 50;
    let mut display = Display::new(
        DISPLAY_X,
        DISPLAY_Y,
        Point(0.0, 0.0, 10.0),
        Point(0.0, 0.0, -1.0),
        3.14 / 2.0,
    );
    let mut cube = Shape::generate_cube_filled(Point(0.0, 0.0, 0.0), 5.0);
    // cube.faces = vec![Face(0, 1, 2), Face(0, 2, 3), Face()];
    Display::clear_screen();
    loop {
        cube.rotate(
            &cube.center.clone().unwrap_or(Point(0.0, 0.0, 0.0)),
            (0.05, 0.05, 0.0),
        );

        display.render_terminal(&cube);

        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
