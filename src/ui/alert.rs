use crate::ui::dialog::Dialog;

use egui::{Context, Align, Layout, Window};

use std::any::Any;

pub struct Alert {
    pub title: String,
    pub message: String,
}

impl Alert {
    pub fn new(title: String) -> Self {
        Self {
            title,
            message: String::new(),
        }
    }

    pub fn reset(&mut self) {
        self.message = String::new();
    }
}

impl Dialog for Alert {
    fn show(&mut self, ctx: &Context, open: &mut bool) {
        Window::new(self.title.as_str())
            .open(open)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label(self.message.as_str());

                    ui.horizontal(|ui| {
                        ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                            if ui.button("OK").clicked() {
                                self.reset();
                            }
                        });
                    });
                });
            });

        if !*open {
            self.reset();
        }
    }

    fn changed(&self) -> (bool, &str) {
        (!self.message.is_empty(), "")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
