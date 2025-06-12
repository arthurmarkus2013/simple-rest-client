use std::vec;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use crossterm::event::{self, KeyCode, Event, KeyModifiers};
use ratatui::style::Stylize;

use crate::config;

#[derive(Default)]
pub struct CreateMovieDialog {
    close: bool,
    updateMode: bool,
    movie: Option<crate::data_types::Movie>,
}

impl CreateMovieDialog {
    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal, updateMode: bool, movie:  Option<crate::data_types::Movie>) -> anyhow::Result<()> {
        self.updateMode = updateMode;
        self.movie = movie;

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
        let title = match self.updateMode {
            true => Line::from("Update Movie".bold()),
            false => Line::from("Create Movie".bold()),
        };

        let mut instructions_list = vec![
            "<ENTER>".bold(),
            "Cancel".into(),
            "<ESC>".bold(),
        ];

        if self.updateMode {
            instructions_list.insert(0, "Update Movie".into());
        } else {
            instructions_list.insert(0, "Create Movie".into());
        }

        let instructions = Line::from(instructions_list);

        let main_window = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(symbols::border::PLAIN);

        let dialog_rect = self.calc_dialog_rect(Size::new(200, 150), frame);

        frame.render_widget(main_window, dialog_rect);
    }

fn handle_event(&mut self, event: Event) {
    if let Event::Key(key_event) = event {
        match key_event.code {
            //
            _ => {
            }
        }
    }
    }

    fn calc_dialog_rect(&self, dialog_size: Size, frame: &Frame) -> Rect {
        let mut dialog_rect = frame.area();

        dialog_rect.x = (dialog_rect.width / 2) - (dialog_size.width / 2);
        dialog_rect.y = (dialog_rect.height / 2) - (dialog_size.height / 2);
        
        dialog_rect
    }
}