mod data_types;

#[path = "config.rs"]
mod config;

pub struct DataLayer {
    config: config::Config,
    movies: Vec<data_types::Movie>
}

impl DataLayer {
    pub fn create_movie(&self, movie: data_types::Movie) {
        todo!()
    }
    pub fn list_movies(&self, id: Option<i32>) -> Vec<data_types::Movie> {
        todo!()
    }

    pub fn update_movie(&self, movie: data_types::Movie) {
        todo!()
    }

    pub fn delete_movie(&self, movie: data_types::Movie) {
        todo!()
    }
}
