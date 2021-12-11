use fltk::{prelude::*,*,group::*};
// use fltk_flex::{Flex,FlexType};
fn main(){
let app = app::App::default();
let mut win = window::Window::
new(100,100,600,600,"Hulk Setup");
win.make_resizable(true);
//-------------------------
let mut flex = Flex::default().size_of_parent().column();
flex.set_pad(10);
    let _expanding = button::Button::default().with_label("Expanding");
    let mut normal = button::Button::default().with_label("Normal");
    let mut normal02 = button::Button::default().with_label("Normal");
    // flex.set_size(&mut normal, 30);
    flex.end();

// let btn_one = btn_middle(&win);
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
