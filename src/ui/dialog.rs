pub trait Dialog {
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}

pub trait Callback {
    fn register_callback(&mut self, callback: Box<dyn FnMut()>);
}
