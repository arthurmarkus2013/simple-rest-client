mod ui;

#[tokio::main]
async fn main() -> iced::Result {

    iced::application("Simple REST Client", ui::MainUi::update, ui::MainUi::view)
    .window_size(iced::Size::new(1100.0, 700.0)).run()
}
