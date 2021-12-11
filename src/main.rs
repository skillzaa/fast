#![allow(unused_mut)]

use fltk::{prelude::*, enums::*, *};
use fltk_flow::Flow;

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);

    let mut win = window::Window::default().with_size(640, 480);
    let mut flow = Flow::default_fill();
    //============== widgets
//......................urow


    // let mut f = frame::Frame::default()
    //     .with_label("Username:");
    // f.set_label_size(25);
       let f = 
       written_text("delet ethe text",44); 


//......................urow

    let inp = input::Input::default().with_size(150, 30);
    let btn_change_data = button::Button::default().with_size(100, 30).with_label("Change");
    //--this is the seperator
    let mut sep = frame::Frame::default().with_size(20, 2);


    
    flow.end();
    win.end();
    win.resizable(&flow);
    win.show();
    
    sep.set_color(Color::Black);
    sep.set_frame(FrameType::FlatBox);
   
    
//=< means expand to left
flow.rule(&inp, "^<=>");
flow.rule(&btn_change_data, "^<");
flow.rule(&sep, "=<^");
flow.rule(&f, "=<^");
    
    a.run().unwrap();
}


fn event_handler(msg:String){
println!("event_handler message:: {}",msg);
}

fn written_text(txt:&str,size:i32)->frame::Frame{

    let mut f = frame::Frame::default()
        .with_label(txt);
       f.set_label_size(size);
f
}