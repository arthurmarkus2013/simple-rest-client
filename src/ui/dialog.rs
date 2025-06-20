pub trait Dialog {
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}
