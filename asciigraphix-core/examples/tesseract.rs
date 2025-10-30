use asciigraphix_core::{
    shapes::{Point, Point4, Shape4}, Display
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
    let l = 2.0;
    let center = Point4(-5.0,-5.0,-5.0,-5.0)*l;
    let mut tesseract = Shape4::generate_4d_paralellepiped(center, Point4(10.0, 0.0, 0.0,0.0)*l, Point4(0.0,10.0,0.0,0.0)*l, Point4(0.0,0.0,10.0,0.0)*l, Point4(0.0,0.0,0.0,10.0)*l);

    // tesseract.rotate(&Point4(0.0, 0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 0.52, 0.51, 0.00));
    loop {
        // tesseract.rotate(&Point4(0.0, 0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 0.02, 0.01, 0.00));
        tesseract.rotate(&Point4(0.0, 0.0, 0.0, 0.0), (0.1, 0.0, 0.0, 0.00, 0.00, 0.00));

        display.render_print(&tesseract.project_to_3d());

        std::thread::sleep(std::time::Duration::from_millis(33));
    }
}
