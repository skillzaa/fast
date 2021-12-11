use fltk::{prelude::*,*,group::*};
// use fltk_flex::{Flex,FlexType};
fn main(){
let app = app::App::default();
let mut win = window::Window::
new(100,100,600,600,"Hulk Setup");
win.make_resizable(true);
//-------------------------
let mut scroll = Scroll::
new(100,100,300,200,"Scroll");

let btn_one = btn_middle(10,10);
let btn_two = btn_middle(40,40);
   

    scroll.end();

//-------------------------
win.end();
win.show();
app.run().unwrap();
}

fn btn_middle(x:i32,y:i32)->button::Button{
  let mut b = button::Button::
  new(x,y,600,400,"Button");
  //-- call back
  b.set_callback(move |b|{
    b.set_label("clicked");
    event_handler();
    // println!("{:?}",b.label()); //dont delete
  });
  b
}

fn event_handler(){
println!("ok");
}
