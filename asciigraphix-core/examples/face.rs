use std::io::Read;

use asciigraphix_core::{
    Display,
    math::Point,
    shapes::{Face, Shape},
};

fn main() {
    const DISPLAY_X: usize = 100;
    const DISPLAY_Y: usize = 50;
    let mut display = Display::new(
        DISPLAY_X,
        DISPLAY_Y,
        Point(0.0, 0.0, 2.0),
        Point(0.0, 0.0, -1.0),
        3.14 / 3.0,
    );
    // let shape = Shape::new(
    //     vec![
    //         Point(-3.0, 0.0, 0.0),
    //         Point(2.0, 2.0, 0.0),
    //         Point(2.0, -2.0, 0.0),
    //     ],
    //     vec![],
    //     vec![Face::new(0,1,2)],
    // );
    let mut shape = Shape::new(
        vec![
            Point(0.0, 0.0, 0.0),
            Point(2.0, 1.0, 0.0),
            Point(1.0, 2.0, 0.0),
        ],
        vec![],
        vec![Face::new(0, 1, 2)],
    );
    // let mut cube = Shape::generate_cube(Point(0.0, 0.0, 0.0), 1.0);
    // cube.faces = vec![Face(0, 1, 2)];

    // display.render_print(&shape.combine(&shape2));
    Display::clear_screen();
    let mut buf = [0u8; 1];
    // loop {
        // shape.rotate(&Point(0.0, 0.0, 0.0), (0.1, 0.0, 0.0));
        display.render_print(&shape);

        // std::io::stdin().read(&mut buf).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(33));
    // }
}
