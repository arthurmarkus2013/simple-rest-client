mod communication_layer;
mod config;
mod data_types;
mod ui;

fn main() -> Result<(), eframe::Error> {
    let mut viewport = egui::ViewportBuilder::default()
        .with_min_inner_size([500.0, 400.0])
        .with_max_inner_size([500.0, 400.0])
        .with_inner_size([500.0, 400.0]);

    viewport.resizable = Some(false);
    viewport.maximize_button = Some(false);

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Simple REST Client",
        options,
        Box::new(|_| Ok(Box::<ui::MainUi>::new(ui::MainUi::new()))),
    )
}
