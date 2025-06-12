use std::vec;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use crossterm::event::{self, KeyCode, Event, KeyModifiers};
use ratatui::style::Stylize;

use crate::config;
use crate::ui::create_movie::CreateMovieDialog;
use crate::ui::login::LoginDialog;
use crate::ui::register::RegisterDialog;

mod login;
mod register;
mod create_movie;

#[derive(Default)]
pub struct Ui {
    exit: bool,
}

impl Ui {
    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> anyhow::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            if let Ok(true) = crossterm::event::poll(std::time::Duration::from_millis(100)) {
                if let Ok(event) = crossterm::event::read() {
                    self.handle_event(event, terminal);
                }
            }
        }

        Ok(())
    }

    fn draw(&self, frame: &mut ratatui::Frame) {
        let title = Line::from("Simple REST Client".bold());
        let mut instructions_list = vec![
            "Register".into(),
            "<Ctrl+R>".bold(),
            "<Ctrl+L>".bold(),
            "Create Movie".into(),
            "<Alt+N>".bold(),
            "Update Movie".into(),
            "<Alt+U>".bold(),
            "Delete Movie".into(),
            "<Alt+D>".bold(),
            "Exit ".into(),
            "<Ctrl+C>".bold(),
        ];

        if config::Config::load_config().current_session_state() == crate::data_types::SessionState::Unauthenticated {
            instructions_list.insert(2, "Login".into());
        } else {
            instructions_list.insert(2, "Logout".into());
        }

        let instructions = Line::from(instructions_list);

        let main_window = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(symbols::border::PLAIN);

        frame.render_widget(main_window, frame.area());
    }

fn handle_event(&mut self, event: Event, terminal: &mut ratatui::DefaultTerminal) {
    if let Event::Key(key_event) = event {
        match key_event.code {
            KeyCode::Char('r') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                RegisterDialog::default().run(terminal)
                    .expect("Failed to run RegisterDialog");
            }
            KeyCode::Char('l') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                // Handle login/logout action
                let config = config::Config::load_config();
                if config.current_session_state() == crate::data_types::SessionState::Unauthenticated {
                    LoginDialog::default().run(terminal)
                        .expect("Failed to run LoginDialog");
                } else {
                    // Handle logout logic
                }
            }
            KeyCode::Char('n') if key_event.modifiers.contains(KeyModifiers::ALT) => {
                CreateMovieDialog::default().run(terminal, false, None)
                    .expect("Failed to run CreateMovieDialog");
            }
            KeyCode::Char('u') if key_event.modifiers.contains(KeyModifiers::ALT) => {
                CreateMovieDialog::default().run(terminal, true, None)
                    .expect("Failed to run UpdateMovieDialog");
            }
            KeyCode::Char('d') if key_event.modifiers.contains(KeyModifiers::ALT) => {
                // Handle delete movie action
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.exit = true;
            }
            _ => {
            }
        }
    }
    }
}