use crate::{data_types::Movie, ui::dialog::{Callback, Dialog}};

pub struct CreateMovieDialog {
    movie: Movie,
    update_mode: bool,
    callback: Box<dyn FnMut()>,
}

impl CreateMovieDialog {
    pub fn new(movie: Option<Movie>) -> Self {
        match movie {
            Some(m) => Self {
                movie: m,
                update_mode: true,
                callback: Box::new(|| {}),
            },
            None => Self {
                movie: Movie::default(),
                update_mode: false,
                callback: Box::new(|| {}),
            },
        }
    }

    pub fn get_movie(&self) -> Option<Movie> {
        if self.valid() {
            Some(self.movie.clone())
        } else {
            None
        }
    }

    fn valid(&self) -> bool {
        !self.movie.title.is_empty() && !self.movie.description.is_empty() && self.movie.release_year > -1
    }
}

impl Callback for CreateMovieDialog {
    fn register_callback(&mut self, callback: Box<dyn FnMut()>) {
        self.callback = callback;
    }
}

impl Dialog for CreateMovieDialog {
    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        let title = if self.update_mode { "Update" } else { "Create" };

        egui::Window::new(title.to_owned() + " Movie")
            .open(open)
            .show(ctx, |ui| {
                ui.label("Create a new movie");

                ui.horizontal(|ui| {
                    ui.label("Title:");
                    ui.text_edit_singleline(&mut self.movie.title).highlight();
                });

                ui.horizontal(|ui| {
                    ui.label("Description:");
                    ui.text_edit_multiline(&mut self.movie.description)
                        .highlight();
                });

                ui.horizontal(|ui| {
                    ui.label("Release Year:");
                    ui.add(
                        egui::DragValue::new(&mut self.movie.release_year)
                            .range(1900..=2100)
                            .speed(1.0)
                            .suffix(" Year"),
                    );
                });

                if ui.button(title).clicked() {
                    if self.valid() {
                        self.callback.as_mut()();
                    }
                }
            });
    }
}
