The .with_label() etc are just for being used with default().

- with set_label we can set label of win or button etc
-     // println!("{:?}",b.label()); //dont delete
- btn.clear_visible_focus();
- btn.set_color();
- btn.set_color().inactive/darker/lighter();
- btn.set_selection_color()
- btn.set_frame() // any windowed item frame can be changed ????
- fltk-theme crate is for theming
- fltk_flex crate provides flex box capabilities
use fltk_fllex::{Flex,FlexType};
- fltk-event crate