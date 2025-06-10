mod ui;
mod data_types;
mod config;
mod communication_layer;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(ui::draw).expect("Failed to draw frame");

        if matches!(event::read().expect("Failed to read event"), Event::Key(event::KeyEvent {
            modifiers: KeyModifiers::CONTROL,
            code: KeyCode::Char('c'),
            ..
        })) {
            break;
        }
    }

    ratatui::restore();

    Ok(())
}
