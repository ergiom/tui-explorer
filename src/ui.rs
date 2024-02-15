use ratatui::{
    style::{Color, Style}, text::{Line, Span}, widgets::List, Frame
};

use crate::{app::App, file_types::FileType};

pub fn render(app: &mut App, frame: &mut Frame) {
    let items: Vec<Line> = app.current_dir_items().into_iter()
        .map(|(file_type, file_name)| {
            Line::default().spans(vec![
                file_type_to_span(file_type),
                Span::default().content(file_name),
            ])
        })
        .collect();

    frame.render_stateful_widget(
        List::new(items)
            .highlight_style(Style::default()
                .bg(Color::Gray)
                .fg(Color::Black)),
        frame.size(),
        &mut app.list_state,
    );
}

fn file_type_to_span(file_type: FileType) -> Span<'static> {
    match file_type {
        FileType::FILE => Span::default().content("F "),
        FileType::DIR => Span::default().content("D "),
        FileType::SYMLINK => Span::default().content("L "),
        FileType::OTHER => Span::default().content("  "),
    }
}
