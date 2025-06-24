use egui_table::TableDelegate;

use crate::data_types::Movie;

pub struct Delegate {
    pub movies: Vec<Movie>,
}

impl Delegate {
    pub fn new() -> Self {
        Self { 
            movies: Vec::new(),
        }
    }
}

impl TableDelegate for Delegate {
    fn header_cell_ui(&mut self, ui: &mut egui::Ui, cell: &egui_table::HeaderCellInfo) {
        ui.horizontal(|ui| {
            ui.label("Movie Name");
            ui.label("Description");
            ui.label("Release Year");
        });
    }

    fn cell_ui(&mut self, ui: &mut egui::Ui, cell: &egui_table::CellInfo) {
        //
    }
}
