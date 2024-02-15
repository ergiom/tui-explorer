use ratatui::{
    widgets::List,
    Frame,
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    frame.render_widget(
        List::new(app.current_dir_items()),
        frame.size()
    );
}
