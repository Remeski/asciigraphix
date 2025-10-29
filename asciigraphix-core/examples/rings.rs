use asciigraphix_core::{
    shapes::{Point, Shape}, Display
};

fn main() {
    const DISPLAY_X: usize = 100;
    const DISPLAY_Y: usize = 50;
    let mut display = Display::new(
        DISPLAY_X,
        DISPLAY_Y,
        Point(-20.0, -5.0, -30.0),
        Point(0.0, 0.0, 1.0),
        20.0,
    );
    let mut ring2 = Shape::generate_ring(5.0, Point(30.0, 5.0, 1.0));
    let mut ring1 = Shape::generate_ring(10.0, Point(-10.0, 5.0, 1.0));
    let mut ring3 = Shape::generate_ring(15.0, Point(-55.0, 5.0, 1.0));
    let mut b: f64 = 0.0;
    loop {
        ring1.rotate(
            &ring1.center.clone().unwrap_or(Point(0.0, 0.0, 0.0)),
            (-0.1 * b.sin(), 0.1 * (1.0 - b.sin()), 0.0),
        );
        ring2.rotate(
            &ring2.center.clone().unwrap_or(Point(0.0, 0.0, 0.0)),
            // (0.1 * b.sin(), 0.1 * (1.0 - b.cos()), 0.0),
            (-0.1 * b.sin(), 0.1 * (1.0 - b.sin()), 0.0),
        );
        ring3.rotate(
            &ring3.center.clone().unwrap_or(Point(0.0, 0.0, 0.0)),
            // (0.01, 0.01 * b.sin(), 0.0),
            (-0.1 * b.sin(), 0.1 * (1.0 - b.sin()), 0.0),
        );
        display.render_print(&ring1.combine(&ring2).combine(&ring3));
        std::thread::sleep(std::time::Duration::from_millis(33));
        b += 0.01;
    }
}
