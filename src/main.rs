mod ui;
mod data_types;
mod config;
mod communication_layer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();

    let result = ui::Ui::default().run(&mut terminal);

    ratatui::restore();

    result
}
