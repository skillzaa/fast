use fltk::{prelude::*, *};

fn main() {
    let app = app::App::default();

    let mut my_window = window::Window::new(100, 100, 400, 300, "My Window");
    
    let mut but = button::Button::new(160, 200, 80, 40, "Click me!");
    
    my_window.end();
    
    my_window.show();
    
    // but.set_callback(|b| b.set_label("Clicked!"));
    but.set_callback(|_b| 
        callback());
    app.run().unwrap();
}

fn callback(){
    println!("some thing some thing");
}
