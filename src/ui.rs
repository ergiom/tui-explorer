use ratatui::{
    style::{Style, Styled}, widgets::List, Frame
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    frame.render_stateful_widget(
        List::new(app.current_dir_items())
            .highlight_symbol(">>"),
        frame.size(),
        &mut app.list_state,
    );
}
