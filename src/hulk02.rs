use fltk::{prelude::*,*,group::*};
fn main(){
let app = app::App::default();
let mut win = window::Window::
new(100,100,800,900,"Hulk Setup");
win.make_resizable(true);
//-------------------------
 
let card01 = card(0);
// card01.center_of_parent();
//-------------------------
win.end();
win.show();
app.run().unwrap();
}
fn card(y:i32)->group::Group{
  let x = 20;
  let mut pack = group::Group::default()
  .with_size(400,300)
  .with_pos(x, y);

  // pack.set_type(group::PackType::Vertical);
  // pack.set_spacing(10);
  let mut input = input::Input::
  new(x, y+20, 350, 50, "Input");
  //input.center_of_parent();
  

  let mut btn = button::Button::
  new(x,y+200,50,40,"ok");
  // btn.center_of_parent();
    btn.set_callback( move |_b|{
      event_handler("button clicked".to_string());
      dialog::message(200, 200, "Waooo");
    });
  pack.end();
  // pack.auto_layout();  
  pack
}


fn event_handler(msg:String){
println!("event_handler message:: {}",msg);
}
