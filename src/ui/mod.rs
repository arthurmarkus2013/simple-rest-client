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

pub struct MainUi<'a> {
    dialogs: Vec<Box<dyn Dialog>>,
    data_layer: &'a mut DataLayer,
    show_dialog: bool,
}

impl<'a> MainUi<'a> {
    pub fn new<'b: 'a>(data_layer: &'b mut DataLayer) -> Self {
        Self {
            dialogs: Vec::new(),
            data_layer: data_layer,
            show_dialog: false,
        }
    }

    fn show_dialog(&mut self, dialog: Box<dyn Dialog>) {
        if !self.show_dialog {
            self.dialogs.clear();
            self.dialogs.push(dialog);
            self.show_dialog = true;
        }
    }
}

impl<'a> eframe::App for MainUi<'a> {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame {
                fill: egui::Color32::from_white_alpha(255),
                ..Default::default()
            })
            .show(ctx, |ui| {
                let mut server_url = String::new();

                ui.horizontal(|ui| {
                    ui.label("Server URL:");
                    if ui
                        .text_edit_singleline(&mut server_url)
                        .highlight()
                        .lost_focus()
                    {
                        self.data_layer.config.base_url = server_url.clone();
                        self.data_layer.config.store_config();
                    }

                    if ui.button("Register").clicked() {
                        let dialog = RegisterDialog::default();
                        
                        if let Some((username, password, role)) = dialog.get_data() {
                            //
                        }
                        
                        self.show_dialog(Box::new(dialog));
                    }

                    if Config::load_config().current_session_state()
                        == data_types::SessionState::Unauthenticated
                    {
                        if ui.button("Login").clicked() {
                            let dialog = LoginDialog::new();
                            
                            if let Some(creds) = dialog.get_credentials() {
                                //
                            }
                            
                            self.show_dialog(Box::new(dialog));
                        }
                    } else {
                        if ui.button("Logout").clicked() {
                            //
                        }
                    }

                    if ui.button("Create Movie").clicked() {
                        let dialog = CreateMovieDialog::new(None);
                        self.show_dialog(Box::new(dialog));
                    }

                    if ui.button("List Movies").clicked() {
                        //
                    }

                    if ui.button("Update Movie").clicked() {
                        let dialog = CreateMovieDialog::new(Some(data_types::Movie::default()));
                        self.show_dialog(Box::new(dialog));
                    }

                    if ui.button("Delete Movie").clicked() {
                        //
                    }
                });
            });

        for dialog in self.dialogs.iter_mut() {
            dialog.show(ctx, &mut self.show_dialog);
        }
    }
}
