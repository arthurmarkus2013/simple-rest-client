use crate::{data_types::Movie, ui::dialog::Dialog};

use egui::{Align, Context, DragValue, Layout, Window};
use std::any::Any;

pub struct CreateMovieDialog {
    movie: Movie,
    update_mode: bool,
    changed: bool,
}

impl Clone for CreateMovieDialog {
    fn clone(&self) -> Self {
        Self {
            movie: self.movie.clone(),
            update_mode: self.update_mode,
            changed: self.changed,
        }
    }
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
        if self.valid() {
            Some(self.movie.clone())
        } else {
            None
        }
    }

    fn valid(&self) -> bool {
        !self.movie.title.is_empty()
            && !self.movie.description.is_empty()
            && self.movie.release_year > -1
    }
}

impl Dialog for CreateMovieDialog {
    fn show(&mut self, ctx: &Context, open: &mut bool) {
        let title = if self.update_mode { "Update" } else { "Create" };

        Window::new(title.to_owned() + " Movie")
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
                        DragValue::new(&mut self.movie.release_year)
                            .range(1900..=2100)
                            .speed(1.0)
                            .suffix(" Year"),
                    );
                });

                ui.horizontal(|ui| {
                    ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                        if ui.button(title).clicked() {
                            if self.valid() {
                                self.changed = true;
                            }
                        }
                    });
                });
            });
    }

    fn changed(&self) -> (bool, &str) {
        if self.update_mode {
            (self.changed, "Update Movie")
        } else {
            (self.changed, "Create Movie")
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
