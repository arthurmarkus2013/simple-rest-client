pub trait Dialog {
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
    fn changed(&self) -> (bool, &str);
    fn as_any(&self) -> &dyn std::any::Any;
}
