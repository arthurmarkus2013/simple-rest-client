use crate::ui::dialog::Dialog;

use egui::{Align, ComboBox, Context, Layout, Window};

use std::any::Any;

pub struct RegisterDialog {
    username: String,
    password: String,
    role: String,
    changed: bool,
}

impl Clone for RegisterDialog {
    fn clone(&self) -> Self {
        Self {
            username: self.username.clone(),
            password: self.password.clone(),
            role: self.role.clone(),
            changed: self.changed,
        }
    }
}

impl RegisterDialog {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            role: "".to_string(),
            changed: false,
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

impl Dialog for RegisterDialog {
    fn show(&mut self, ctx: &Context, open: &mut bool) {
        Window::new("Register").open(open).show(ctx, |ui| {
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
                ComboBox::from_label("")
                    .selected_text(&self.role)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.role, "Admin".to_string(), "Admin");
                        ui.selectable_value(&mut self.role, "User".to_string(), "User");
                    });
            });

            ui.horizontal(|ui| {
                ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                    if ui.button("Register").clicked() {
                        if self.valid() {
                            self.changed = true;
                        }
                    }
                });
            });
        });
    }

    fn changed(&self) -> (bool, &str) {
        (self.changed, "Register")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
