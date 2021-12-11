use fltk::{prelude::*,*};

fn main(){
let app = app::App::default();
let mut win = window::Window::
new(100,100,600,600,"Hulk Setup");
//-------------------------
let btn_one = btn_middle(&win);
//-------------------------
win.end();
win.show();
app.run().unwrap();
}

fn btn_middle(win:&window::Window)->button::Button{
  let mut b = button::Button::
  default()
  .with_size(70, 30)
  .with_label("Set")
  .center_of(win);
  //-- call back
  b.set_callback(move |b|{
    b.set_label("clicked");
    // println!("{:?}",b.label()); //dont delete
  });
  b
}
