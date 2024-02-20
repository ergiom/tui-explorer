use std::os::windows::fs::MetadataExt;

use ratatui::{
    layout::{Constraint, Layout, Rect}, style::{Color, Style}, text::{Line, Span}, widgets::{Block, Borders, List, Paragraph}, Frame
};

use crate::{app::App, file_types::FileType};

pub fn render(app: &mut App, frame: &mut Frame) {
    draw_main_view(app, frame);
    if app.has_error() {
        draw_error_popup(app, frame);
    }
    else if app.show_details {
        draw_details(app, frame);
    }
}

fn draw_details(app: &mut App, frame: &mut Frame) {
    if let Ok(entry) = app.details() {
        let outer = Block::new().borders(Borders::ALL).title(entry.file_name().into_string().expect("Couldn't parse item's name"));
        let popup_area = popup(frame.size(), 40, 40);
        let inner_area = outer.inner(popup_area);

        frame.render_widget(outer, popup_area);

        if let Ok(metadata) = entry.metadata() {
            let list = List::new(vec![
                format!("Size: {}", metadata.file_size()),
                format!("Creation time: {}", metadata.creation_time()),
                format!("Last access: {}", metadata.last_access_time()),
            ]);
            frame.render_widget(list, inner_area);
        }
    }
    else {

    }
}

fn draw_error_popup(app: &mut App, frame: &mut Frame) {
    let popup = popup(frame.size(), 40, 40);
    let outer_block = Block::default()
        .borders(Borders::ALL)
        .title("Error");

    let inner = outer_block.inner(popup);
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Percentage(80),
            Constraint::Min(1),
        ])
        .split(inner);

    frame.render_widget(outer_block, popup);

    frame.render_widget(
        Paragraph::new(app.error.clone())
            .alignment(ratatui::layout::Alignment::Center),
        layout[0],
    );
    frame.render_widget(
        Paragraph::new("OK")
            .alignment(ratatui::layout::Alignment::Center),
        layout[1],
    );
}

fn draw_main_view(app: &mut App, frame: &mut Frame) {
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
