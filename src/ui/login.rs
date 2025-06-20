use crate::{data_types::Credentials, ui::dialog::{Dialog}};

pub struct LoginDialog {
    creds: Credentials,
    changed: bool,
}

impl LoginDialog {
    pub fn new() -> Self {
        Self {
            creds: Credentials::default(),
            changed: false,
        }
    }
}

impl LoginDialog {
    pub fn get_credentials(&self) -> Option<Credentials> {
        if self.changed {
            Some(self.creds.clone())
        } else {
            None
        }
    }
}

impl Dialog for LoginDialog {
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new("Login").open(open).show(ctx, |ui| {
            ui.label("Login to your account");

            ui.horizontal(|ui| {
                ui.label("Username:");
                ui.text_edit_singleline(&mut self.creds.username);
            });

            ui.horizontal(|ui| {
                ui.label("Password:");
                ui.text_edit_singleline(&mut self.creds.password);
            });

            if ui.button("Login").clicked() {
                if self.changed {
                    //
                }
            }
        });
    }
}
