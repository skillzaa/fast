use fltk::{prelude::*, *};

fn main() {
    let app = app::App::default();

    let mut win = window::Window::new(250,250,500,650,"Hulk Settings");
    win.make_resizable(true);

    let gp = group::Group::
    new(20,20,400,500,"Group 01");
      get_gp_content();
    gp.end();

    win.end();
    win.show();
    
   


    app.run().unwrap();
}

fn callback(height:&str){
    println!("This height of the button is: {}",height);
}

fn get_gp_content()->group::Pack{

    let mut pack = group::Pack::default().with_size(290, 80);
    
    pack.set_type(group::PackType::Horizontal);
    
    pack.set_spacing(90);
    
     let input02 = fltk::input::Input::
     new(20,20,150,40,"Text Area");
     //input.center_of(&win);
 
     let mut btn02 = button::Button::new(25,250,100,75,"Bingoo!!!");
     // btn.center_of(&win);
     
      // but.set_callback(|b| b.set_label("Clicked!"));
    btn02.set_callback(move |_b| 
        {
           // b.set_label("new_title");

           callback(&input02.value());
        });
pack.end();
pack
}