use crate::{data_types::Credentials, ui::dialog::Dialog};

#[derive(Default)]
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
                if self.valid() {
                    self.changed = true;
                }
            }
        });
    }

    fn changed(&self) -> (bool, &str) {
        (self.changed, "Login")
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
