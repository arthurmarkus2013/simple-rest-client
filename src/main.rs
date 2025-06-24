mod communication_layer;
mod config;
mod data_types;
mod ui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Simple REST Client",
        options,
        Box::new(|_| Ok(Box::<ui::MainUi>::new(ui::MainUi::new()))),
    )
}
