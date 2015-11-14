use command::prelude::*;
use datatypes::{Area, Style};

#[derive(Copy, Clone)]
pub struct SetStyleInArea(pub Area, pub Style);

impl Command for SetStyleInArea {
    fn apply(&self, terminal: &mut Terminal) {
        terminal.set_style_in_area(self.0, self.1);
    }
    fn repr(&self) -> String {
        String::from("SET STYLE IN AREA")
    }
}

#[derive(Copy, Clone)]
pub struct DefaultStyleInArea(pub Area);

impl Command for DefaultStyleInArea {
    fn apply(&self, terminal: &mut Terminal) {
        terminal.reset_styles_in_area(self.0);
    }
    fn repr(&self) -> String {
        String::from("DEFAULT STYLE IN AREA")
    }
}
