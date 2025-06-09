use iced::widget::{Column, column, text};

#[path = "data_layer.rs"]
mod data_layer;

#[derive(Debug, Clone, Copy)]
pub enum Message {}

#[derive(Default)]
pub struct MainUi {}

impl MainUi {
    pub fn view(&self) -> Column<Message> {
        column![text("Hello, World!").size(50)]
    }

    pub fn update(&mut self, message: Message) {
        //
    }
}
