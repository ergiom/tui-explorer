use ratatui::{
    layout::{Constraint, Layout, Rect}, style::{Color, Style}, text::{Line, Span}, widgets::{Block, Borders, List, Paragraph}, Frame
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

    if app.has_error() {
        frame.render_widget(
            Paragraph::new("OK")
                .alignment(ratatui::layout::Alignment::Center)
                .block(Block::default().title("Error").borders(Borders::ALL)),
            popup(frame.size(), 40, 40)
        );
    }
}

fn file_type_to_span(file_type: FileType) -> Span<'static> {
    match file_type {
        FileType::FILE => Span::default().content("F "),
        FileType::DIR => Span::default().content("D "),
        FileType::SYMLINK => Span::default().content("L "),
        FileType::OTHER => Span::default().content("  "),
    }
}

fn popup(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(popup_layout[1])[1]
}
