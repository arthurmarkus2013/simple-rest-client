use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

use egui::{Layout, Sense};
use egui_extras::{Column, TableBuilder};

use crate::{
    communication_layer::DataLayer,
    config::Config,
    data_types::{self, Role},
    ui::{
        create_movie::CreateMovieDialog, dialog::Dialog, login::LoginDialog,
        register::RegisterDialog,
    },
};

mod create_movie;
mod dialog;
mod login;
mod register;

type Callback<T: Dialog> = dyn FnMut(Ref<Box<T>>, RefMut<DataLayer>);

pub struct MainUi {
    dialogs: Vec<RefCell<Box<dyn Dialog>>>,
    data_layer: RefCell<DataLayer>,
    show_dialog: bool,
    callbacks: HashMap<String, Box<Callback<dyn Dialog>>>,
    selected_movie_id: Option<i32>,
}

impl MainUi {
    pub fn new() -> Self {
        Self {
            dialogs: Vec::new(),
            data_layer: RefCell::new(DataLayer::new()),
            show_dialog: false,
            callbacks: HashMap::new(),
            selected_movie_id: None,
        }
    }

    fn show_dialog(&mut self, dialog: Box<dyn Dialog>) {
        if !self.show_dialog {
            self.dialogs.clear();
            self.dialogs.push(RefCell::new(dialog));
            self.show_dialog = true;
        }
    }
}

impl eframe::App for MainUi {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame {
                fill: egui::Color32::from_white_alpha(255),
                ..Default::default()
            })
            .show(ctx, |ui| {
                let mut server_url = String::new();

                ui.vertical(|ui| {
                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        ui.add_space(5.0);

                        ui.label("Server URL:");
                        if ui
                            .text_edit_singleline(&mut server_url)
                            .highlight()
                            .lost_focus()
                        {
                            self.data_layer.borrow_mut().config.base_url = server_url.clone();
                            self.data_layer.borrow_mut().config.store_config();
                        }

                        if ui.button("Register").clicked() {
                            let dialog = RegisterDialog::new();
                            self.show_dialog(Box::new(dialog));

                            self.callbacks.insert(
                                "Register".to_string(),
                                Box::new(|dialog, data_layer| {
                                    let register_dialog =
                                        dialog.as_any().downcast_ref::<RegisterDialog>().unwrap();

                                    let (username, password, role) =
                                        register_dialog.get_data().unwrap();

                                    data_layer.register(username, password, Role::from(role));
                                }),
                            );
                        }

                        if Config::load_config().current_session_state()
                            == data_types::SessionState::Unauthenticated
                        {
                            if ui.button("Login").clicked() {
                                let dialog = LoginDialog::default();
                                self.show_dialog(Box::new(dialog));

                                self.callbacks.insert(
                                    "Login".to_string(),
                                    Box::new(|dialog, mut data_layer| {
                                        let login_dialog =
                                            dialog.as_any().downcast_ref::<LoginDialog>().unwrap();

                                        let credentials = login_dialog.get_credentials().unwrap();

                                        data_layer
                                            .login(credentials.username, credentials.password);
                                    }),
                                );
                            }
                        } else {
                            if ui.button("Logout").clicked() {
                                self.data_layer.borrow_mut().logout();
                            }
                        }
                    });

                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        ui.add_space(5.0);

                        if ui.button("Create Movie").clicked() {
                            let dialog = CreateMovieDialog::new(None);
                            self.show_dialog(Box::new(dialog));

                            self.callbacks.insert(
                                "Create Movie".to_string(),
                                Box::new(|dialog, data_layer| {
                                    let create_movie_dialog = dialog
                                        .as_any()
                                        .downcast_ref::<CreateMovieDialog>()
                                        .unwrap();

                                    let movie = create_movie_dialog.get_movie().unwrap();

                                    data_layer.create_movie(movie);
                                }),
                            );
                        }

                        if ui.button("List Movies").clicked() {
                            self.data_layer.borrow_mut().list_movies(None);
                        }

                        if ui.button("Update Movie").clicked() {
                            let dialog = CreateMovieDialog::new(Some(self.data_layer.borrow().movies.iter()
                                .find(|&movie| movie.id == self.selected_movie_id.unwrap()).unwrap().clone()));

                            self.show_dialog(Box::new(dialog));

                            self.callbacks.insert(
                                "Update Movie".to_string(),
                                Box::new(|dialog, data_layer| {
                                    let update_movie_dialog = dialog
                                        .as_any()
                                        .downcast_ref::<CreateMovieDialog>()
                                        .unwrap();

                                    let movie = update_movie_dialog.get_movie().unwrap();

                                    data_layer.update_movie(movie);
                                }),
                            );
                        }

                        if ui.button("Delete Movie").clicked() {
                            self.data_layer.borrow_mut().delete_movie(self.selected_movie_id.unwrap());
                        }

                        if ui.button("Clear Selection").clicked() {
                            self.selected_movie_id = None;
                        }
                    });

                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.add_space(5.0);

                        ui.with_layout(Layout::centered_and_justified(ui.layout().main_dir())
                        .with_cross_align(egui::Align::Center).with_cross_justify(true), |ui| {
                            TableBuilder::new(ui)
                                .animate_scrolling(true)
                                .striped(true)
                                .column(Column::auto().at_least(100.0))
                                .column(Column::auto().at_least(200.0))
                                .column(Column::auto().at_least(100.0))
                                .sense(Sense::click())
                                .auto_shrink(false)
                                .resizable(false)
                                .header(10.0, |mut header| {
                                    header.col(|ui| {
                                        ui.label("Movie Name");
                                    });
                                    header.col(|ui| {
                                        ui.label("Description");
                                    });
                                    header.col(|ui| {
                                        ui.label("Release Year");
                                    });
                                })
                                .body(|mut body| {
                                    for movie in self.data_layer.borrow().movies.iter() {
                                        body.row(10.0, |mut row| {
                                            row.col(|ui| {
                                                ui.label(&movie.title);
                                            });
                                            row.col(|ui| {
                                                ui.label(&movie.description);
                                            });
                                            row.col(|ui| {
                                                ui.label(&movie.release_year.to_string());
                                            });

                                            if row.response().clicked() {
                                                self.selected_movie_id = Some(movie.id);
                                            }

                                            row.set_selected(self.selected_movie_id == Some(movie.id));
                                        });
                                    }
                                });
                        });
                    });
                });

                for dialog in self.dialogs.iter_mut() {
                    let mut dlg = dialog.borrow_mut();

                    dlg.show(ctx, &mut self.show_dialog);

                    let (changed, dialog_name) = dlg.changed();

                    if changed {
                        self.callbacks.get_mut(dialog_name).map(|callback| {
                            callback(dialog.borrow(), self.data_layer.borrow_mut())
                        });
                    }
                }
            });
    }
}
