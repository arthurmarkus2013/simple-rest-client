mod communication_layer;
mod config;
mod data_types;
mod ui;

use eframe::{Error, NativeOptions, run_native};
use egui::ViewportBuilder;

use ui::MainUi;

fn main() -> Result<(), Error> {
    let mut viewport = ViewportBuilder::default()
        .with_min_inner_size([500.0, 400.0])
        .with_max_inner_size([500.0, 400.0])
        .with_inner_size([500.0, 400.0]);

    viewport.resizable = Some(false);
    viewport.maximize_button = Some(false);

    let options = NativeOptions {
        viewport,
        ..Default::default()
    };

    run_native(
        "Simple REST Client",
        options,
        Box::new(|_| Ok(Box::<MainUi>::new(MainUi::new()))),
    )
}
