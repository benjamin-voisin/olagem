use ratatui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, prelude::Rect,
    prelude::{Span, Line},
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let first_line =
        Line::from(vec![
            Span::styled(&app.correctly_typed, Style::default().fg(Color::Green)),
            Span::styled(&app.wrongly_typed, Style::default().bg(Color::Red).fg(Color::Black)),
            Span::styled(&app.to_type, Style::default().fg(Color::White)),
        ]).alignment(Alignment::Left);
    let second_line =
        Line::styled(&app.second_sentence, Style::default().fg(Color::White))
        .alignment(Alignment::Left);

    let height = frame.size().height;
    let width = frame.size().width;
    frame.render_widget(
        Paragraph::new(vec![first_line, second_line])
        .block(
            Block::default()
                .title("olagem")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Left),
        Rect::new(width/2 - (width / 3), height / 2 - 2, (width / 3) * 2, 4),
    )
}
