use asciigraphix::{
    shapes::{Point, Point4, Shape, Shape4}, Display
};

fn main() {
    const DISPLAY_X: usize = 300;
    const DISPLAY_Y: usize = 100;
    let mut display = Display::new(
        DISPLAY_X,
        DISPLAY_Y,
        Point(0.0, 0.0, -20.0),
        Point(0.0, 0.0, 1.0),
        20.0,
    );
    // let mut ring1 = Shape::generate_ring(10.0, Point(1.0, 1.0, 1.0));
    // let mut cube = Shape::generate_cube(Point(0.0, 0.0, 0.0), 5.0);

    // cube.rotate(&cube.center.clone().unwrap(), (0.2, 0.3, 0.0));
    // let mut line = Shape::generate_line(Point(0.0, 0.0, 0.0), Point(5.0, 0.0, 0.0));
    // let mut grid = Shape::generate_grid(20);
    let mut paralellepiped = Shape::generate_parallelepiped(Point(0.0,0.0,0.0), Point(25.0, 0.0, 0.0), Point(5.0,10.0,5.0), Point(0.0,0.0,10.0));
    let mut b: f64 = 0.0;

    let l = 2.0;
    let center = Point4(-5.0,-5.0,-5.0,-5.0)*l;
    let mut fourd = Shape4::generate_4d_paralellepiped(center, Point4(10.0, 0.0, 0.0,0.0)*l, Point4(0.0,10.0,0.0,0.0)*l, Point4(0.0,0.0,10.0,0.0)*l, Point4(0.0,0.0,0.0,10.0)*l);

        // fourd.rotate(&Point4(0.0, 0.0, 0.0, 0.0), (0.0, 0.5, 0.0, 0.0, 0.0, 0.00));
    loop {
        fourd.rotate(&Point4(0.0, 0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 0.01, 0.01, 0.00));
        paralellepiped.rotate(&Point(5.0,5.0,5.0), (0.0,0.1,0.0));
        // ring1.rotate(
        //     &ring1.center.clone().unwrap_or(Point(0.0, 0.0, 0.0)),
        //     (-0.1 * b.sin(), 0.1 * (1.0 - b.sin())),
        // );
        // ring1.rotate(
        //     &ring1.center.clone().unwrap_or(Point(0.0, 0.0, 0.0)),
        //     (0.0, 0.1, 0.1),
        // );
        // line.rotate(&Point(0.0, 0.0, 0.0), (0.1, 0.0, 0.0));
        // cube.rotate(
        //     &cube.center.clone().unwrap_or(Point(0.0, 0.0, 0.0)),
        //     (0.1, 0.1, 0.0),
        // );
        // cube.rotate(&cube.center.clone().unwrap(), (0.0, 0.1, 0.0));
        // display.render(&line);
        display.render(&fourd.project_to_3d());
        // display.render(&cube);
        // display.render(&cube);
        std::thread::sleep(std::time::Duration::from_millis(33));
        b += 0.01;
    }
}
