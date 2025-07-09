use crate::{data_types::Credentials, ui::dialog::Dialog};

use egui::{Context, Align, Layout, Window};
use std::any::Any;

pub struct LoginDialog {
    creds: Credentials,
    changed: bool,
}

impl Clone for LoginDialog {
    fn clone(&self) -> Self {
        Self {
            creds: self.creds.clone(),
            changed: self.changed,
        }
    }
}

impl LoginDialog {
    pub fn new(creds: Credentials) -> Self {
        Self {
            creds,
            changed: false,
        }
    }

    pub fn get_credentials(&self) -> Option<Credentials> {
        if self.valid() {
            Some(self.creds.clone())
        } else {
            None
        }
    }

    fn valid(&self) -> bool {
        !self.creds.username.is_empty() && !self.creds.password.is_empty()
    }
}

impl Dialog for LoginDialog {
    fn show(&mut self, ctx: &Context, open: &mut bool) {
        Window::new("Login").open(open).show(ctx, |ui| {
            ui.label("Login to your account");

            ui.horizontal(|ui| {
                ui.label("Username:");
                ui.text_edit_singleline(&mut self.creds.username);
            });

            ui.horizontal(|ui| {
                ui.label("Password:");
                ui.text_edit_singleline(&mut self.creds.password);
            });

            ui.horizontal(|ui| {
                ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                    if ui.button("Login").clicked() {
                        if self.valid() {
                            self.changed = true;
                        }
                    }
                });
            });
        });
    }

    fn changed(&self) -> (bool, &str) {
        (self.changed, "Login")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
