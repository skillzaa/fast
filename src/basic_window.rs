use fltk::{prelude::*, *};

fn main() {
    let app = app::App::default();
    let mut my_window = window::Window::new(100, 100, 400, 300, "My Window");
    //--any widgets created between my_window creation and my_window.end() wil be owned by my_window. 
    my_window.end();
    my_window.show();
    app.run().unwrap();
}
