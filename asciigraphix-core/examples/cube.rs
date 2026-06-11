use std::io::Read;

use asciigraphix_core::{
    Display,
    math::Point,
    shapes::{Face, Shape},
};

fn main() {
    const DISPLAY_X: usize = 200;
    const DISPLAY_Y: usize = 100;
    let mut display = Display::new(
        DISPLAY_X,
        DISPLAY_Y,
        Point(0.0, 0.0, 6.0),
        Point(0.0, 0.0, -1.0),
        3.14 / 3.0,
    );


    let mut cube = Shape::generate_cube_colorful(Point(0.0, 0.0, 0.0), 4.0);
    // cube.faces = vec![cube.faces[0..4].to_vec(), cube.faces[6..8].to_vec()]
    //     .concat()
    //     .to_vec();

    Display::clear_screen();
    let mut buf = [0u8; 1];
    loop {
        cube.rotate(
            &cube.center.clone().unwrap_or(Point(0.0, 0.0, 0.0)),
            (0.09, 0.01, 0.00),
        );

        // std::io::stdin().read(&mut buf).unwrap();

        display.render_terminal(&cube);

        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
