pub fn draw(frame: &mut ratatui::Frame) {
    let mut text = ratatui::text::Text::raw("Hello, world!");
    text = text.centered();
    
    frame.render_widget(text, frame.size());
}
