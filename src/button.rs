use fltk::{prelude::*, *};

fn main() {
    let app = app::App::default();
    let mut my_window = window::Window::new(100, 100, 400, 300, "My Window");
    let mut but = button::Button::new(160, 200, 80, 40, "Alhamdullah!");
    my_window.end();
    my_window.show();
    app.run().unwrap();
}
