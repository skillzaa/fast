use fltk::{prelude::*, *};

fn main() {
    let app = app::App::default();
    let mut my_window = window::Window::new(100, 100, 400, 300, "My Window");
    let mut my_window2 = window::Window::new(10, 10, 380, 280, "");
    // my_window2.set_color(Color::Black);
    my_window2.end();
    my_window.end();
    my_window.show();
    app.run().unwrap();
}
