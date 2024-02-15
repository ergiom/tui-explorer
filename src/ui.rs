use ratatui::{
    widgets::Paragraph,
    Frame,
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(format!("{:?}", app.current_dir_items())),
        frame.size()
    );
}
