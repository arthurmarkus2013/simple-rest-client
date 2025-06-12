use std::vec;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use crossterm::event::{self, KeyCode, Event, KeyModifiers};
use ratatui::style::Stylize;

use crate::config;

#[derive(Default)]
pub struct RegisterDialog {
    close: bool,
}

impl RegisterDialog {
    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> anyhow::Result<()> {
        while !self.close {
            terminal.draw(|frame| self.draw(frame))?;

            if let Ok(true) = crossterm::event::poll(std::time::Duration::from_millis(100)) {
                if let Ok(event) = crossterm::event::read() {
                    self.handle_event(event);
                }
            }
        }

        Ok(())
    }

    fn draw(&self, frame: &mut ratatui::Frame) {
        let title = Line::from("Register".bold());

        let instructions = Line::from(vec![
            "Register".into(),
            "<ENTER>".bold(),
            "Cancel".into(),
            "<ESC>".bold(),
        ]);

        let dialog = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(symbols::border::PLAIN);

        let content = Paragraph::default()
            .block(dialog)
            .centered()
            .

        let dialog_rect = self.calc_dialog_rect(Size::new(50, 15), frame);

        frame.render_widget(main_window, dialog_rect);
    }

fn handle_event(&mut self, event: Event) {
    if let Event::Key(key_event) = event {
        match key_event.code {
            KeyCode::Enter => {
                // Handle registration logic here
                self.close = true;
            }
            KeyCode::Esc => {
                self.close = true;
            }
            _ => {
            }
        }
    }
    }

    fn calc_dialog_rect(&self, dialog_size: Size, frame: &Frame) -> Rect {
        let mut dialog_rect = frame.area();

        dialog_rect.x = (dialog_rect.width / 2) - (dialog_size.width / 2);
        dialog_rect.y = (dialog_rect.height / 2) - (dialog_size.height / 2);

        dialog_rect.width = dialog_size.width;
        dialog_rect.height = dialog_size.height;
        
        dialog_rect
    }
}