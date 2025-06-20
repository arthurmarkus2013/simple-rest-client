use crate::{data_types::Movie, ui::dialog::Dialog};

pub struct CreateMovieDialog {
    movie: Movie,
    update_mode: bool,
    changed: bool,
}

impl CreateMovieDialog {
    pub fn new(movie: Option<Movie>) -> Self {
        match movie {
            Some(m) => Self {
                movie: m,
                update_mode: true,
                changed: false,
            },
            None => Self {
                movie: Movie::default(),
                update_mode: false,
                changed: false,
            },
        }
    }

    pub fn get_movie(&self) -> Option<Movie> {
        if self.changed {
            Some(self.movie.clone())
        } else {
            None
        }
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
                    //
                }
            });
    }
}
