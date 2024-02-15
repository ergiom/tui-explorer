use ratatui::{
    style::{Color, Style}, widgets::List, Frame
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    frame.render_stateful_widget(
        List::new(app.current_dir_items())
            .highlight_style(Style::default()
                .bg(Color::Gray)
                .fg(Color::Black)),
        frame.size(),
        &mut app.list_state,
    );
}
