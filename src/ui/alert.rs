use crate::ui::dialog::Dialog;

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
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.title.as_str())
            .open(open)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label(self.message.as_str());

                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
