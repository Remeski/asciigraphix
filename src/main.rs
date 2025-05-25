use asciigraphix::{
    Display,
    shapes::{Point, Shape},
};

fn main() {
    const DISPLAY_X: usize = 100;
    const DISPLAY_Y: usize = 50;
    let mut display = Display::new(DISPLAY_X, DISPLAY_Y);
    let mut ring1 = Shape::generate_ring(10.0, Point(1.0, 1.0, 1.0));
    // let mut line = Shape::generate_line(Point(0.0, 0.0, 0.0), Point(5.0, 0.0, 0.0));
    // let mut b: f64 = 0.0;

    loop {
        // ring1.rotate(
        //     &ring1.center.clone().unwrap_or(Point(0.0, 0.0, 0.0)),
        //     (-0.1 * b.sin(), 0.1 * (1.0 - b.sin())),
        // );
        ring1.rotate(
            &ring1.center.clone().unwrap_or(Point(0.0, 0.0, 0.0)),
            (0.0, 0.1, 0.1),
        );
        // line.rotate(&Point(0.0, 0.0, 0.0), (0.1, 0.0, 0.0));
        // display.render(&line);
        display.render(&ring1);
        std::thread::sleep(std::time::Duration::from_millis(33));
        // b += 0.01;
    }
}
