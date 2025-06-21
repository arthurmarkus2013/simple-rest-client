use egui::Window;

use crate::ui::dialog::{Callback, Dialog};

pub struct RegisterDialog {
    pub username: String,
    pub password: String,
    pub role: String,
    callback: Box<dyn FnMut()>,
}

impl RegisterDialog {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            role: "".to_string(),
            callback: Box::new(|| {}),
        }
    }

    pub fn get_data(&self) -> Option<(String, String, String)> {
        if self.valid() {
            Some((
                self.username.clone(),
                self.password.clone(),
                self.role.clone(),
            ))
        } else {
            None
        }
    }

    fn valid(&self) -> bool {
        !self.username.is_empty() && !self.password.is_empty() && !self.role.is_empty()
    }
}

impl Callback for RegisterDialog {
    fn register_callback(&mut self, callback: Box<dyn FnMut()>) {
        self.callback = callback;
    }
}

impl Dialog for RegisterDialog {
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new("Register").open(open).show(ctx, |ui| {
            ui.label("Register a new user");

            ui.horizontal(|ui| {
                ui.label("Username:");
                ui.text_edit_singleline(&mut self.username).highlight();
            });

            ui.horizontal(|ui| {
                ui.label("Password:");
                ui.text_edit_singleline(&mut self.password).highlight();
            });

            ui.horizontal(|ui| {
                ui.label("Role:");
                egui::ComboBox::from_label("")
                    .selected_text(&self.role)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.role, "Admin".to_string(), "Admin");
                        ui.selectable_value(&mut self.role, "User".to_string(), "User");
                    });
            });

            if ui.button("Register").clicked() {
                if self.valid() {
                    self.callback.as_mut()();
                }
            }
        });
    }
}
