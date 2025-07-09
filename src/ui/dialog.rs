use egui::Context;
use std::any::Any;

pub trait Dialog {
    fn show(&mut self, ctx: &Context, open: &mut bool);
    fn changed(&self) -> (bool, &str);
    fn as_any(&self) -> &dyn Any;
}
