use fltk::{prelude::*, *};

fn main() {
    let app = app::App::default();

    let mut win = window::Window::new(250,250,500,650,"Hulk Settings");

    let mut input = fltk::input::Input::
    new(20,20,150,40,"Text Area");
    //input.center_of(&win);

    let mut btn = button::Button::new(25,250,100,75,"Bingoo!!!");
    // btn.center_of(&win);
    
    win.end();
    
    win.show();
    
    // but.set_callback(|b| b.set_label("Clicked!"));
    btn.set_callback(move |_b| 
        {
           // b.set_label("new_title");

           callback(&input.value());
        });



    app.run().unwrap();
}

fn callback(height:&str){
    println!("This height of the button is: {}",height);
}

