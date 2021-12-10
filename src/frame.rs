use fltk::{prelude::*, *};

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default()
        .with_size(460, 400)
        .center_screen()
        .with_label("Counter");
    let mut frame = frame::Frame::default()
        .with_size(100, 40)
        .center_of(&wind)
        .with_label("0");
    let mut but_inc = button::Button::default()
        .size_of(&frame)
        .above_of(&frame, 0)
        .with_label("+");
    let mut but_dec = button::Button::default()
        .size_of(&frame)
        .below_of(&frame, 0)
        .with_label("-");
    wind.end();
    wind.show();
    app.run().unwrap();
}
