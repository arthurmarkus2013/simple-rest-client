use crate::{data_types::Credentials, ui::dialog::{Callback, Dialog}};

pub struct LoginDialog {
    creds: Credentials,
    callback: Box<dyn FnMut()>,
}

impl LoginDialog {
    pub fn new() -> Self {
        Self {
            creds: Credentials::default(),
            callback: Box::new(|| {}),
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

impl Callback for LoginDialog {
    fn register_callback(&mut self, callback: Box<dyn FnMut()>) {
        self.callback = callback;
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
                    self.callback.as_mut()();
                }
            }
        });
    }
}
