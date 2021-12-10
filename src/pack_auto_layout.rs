use fltk::{prelude::*, *};

fn main() {
    let app = app::App::default();
    
    let mut my_window = window::Window::default().with_size(400, 300);
    let mut hpack = group::Pack::new(0, 200, 400, 100, "");
    hpack.set_type(group::PackType::Horizontal);
    hpack.set_spacing(30);
    let _but1 = button::Button::default().with_label("Button1");
    let _but2 = button::Button::default().with_label("Button2");
    hpack.end();
    hpack.auto_layout();
    my_window.end();
    my_window.show();

    app.run().unwrap();
}
