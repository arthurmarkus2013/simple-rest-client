use egui::Window;

use crate::ui::dialog::Dialog;

#[derive(Default)]
pub struct RegisterDialog {
    pub username: String,
    pub password: String,
    pub role: String,
}

impl RegisterDialog {
    pub fn get_data(&self) -> Option<(String, String, String)> {
        if !self.username.is_empty() && !self.password.is_empty() && !self.role.is_empty() {
            Some((self.username.clone(), self.password.clone(), self.role.clone()))
        } else {
            None
        }
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
                //
            }
        });
    }
}
