use tokio::runtime::Runtime;

mod communication_layer;
mod config;
mod data_types;
mod ui;

fn main() -> Result<(), eframe::Error> {
    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    let _ = rt.enter();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 700.0]),
        ..Default::default()
    };

    let mut data_layer = communication_layer::DataLayer::new();

    eframe::run_native(
        "Simple REST Client",
        options,
        Box::new(|_| Ok(Box::<ui::MainUi>::new(ui::MainUi::new(&mut data_layer)))),
    )
}
