#[derive(Default)]
pub struct MainUi {
    exit: bool,
}

impl eframe::App for MainUi {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().frame(egui::Frame {
            fill: egui::Color32::from_white_alpha(255),
            ..Default::default()
        }).show(ctx, |ui| {
            //
        });
    }
}
