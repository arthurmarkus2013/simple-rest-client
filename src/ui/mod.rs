use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap, ops::DerefMut,
};

use egui_table::Table;

use crate::{
    communication_layer::DataLayer,
    config::Config,
    data_types::{self, Role},
    ui::{
        create_movie::CreateMovieDialog, delegate::Delegate, dialog::Dialog, login::LoginDialog, register::RegisterDialog
    },
};

mod create_movie;
mod dialog;
mod login;
mod register;

mod delegate;

type Callback<T: Dialog> = dyn FnMut(Ref<Box<T>>, RefMut<DataLayer>);

pub struct MainUi {
    dialogs: Vec<RefCell<Box<dyn Dialog>>>,
    data_layer: RefCell<DataLayer>,
    show_dialog: bool,
    callbacks: HashMap<String, Box<Callback<dyn Dialog>>>,
    table_delegate: Delegate,
}

impl MainUi {
    pub fn new() -> Self {
        Self {
            dialogs: Vec::new(),
            data_layer: RefCell::new(DataLayer::new()),
            show_dialog: false,
            callbacks: HashMap::new(),
            table_delegate: Delegate::new(),
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
                            self.table_delegate.movies = self.data_layer.borrow_mut().list_movies(None).unwrap();
                        }

                        if ui.button("Update Movie").clicked() {
                            let dialog = CreateMovieDialog::new(Some(data_types::Movie::default()));
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
                            //
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.add_space(5.0);

                        let table = Table::new();
                        table.show(ui, &mut self.table_delegate);
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
