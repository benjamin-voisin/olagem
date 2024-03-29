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
pub fn render_test<B: Backend>(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let testapp = app.testapp.as_ref().unwrap();
    let first_line =
        Line::from(vec![
            Span::styled(&testapp.correctly_typed, Style::default().fg(Color::Green)),
            Span::styled(&testapp.wrongly_typed, Style::default().bg(Color::Red).fg(Color::Black)),
            Span::styled(&testapp.to_type, Style::default().fg(Color::White)),
        ]).alignment(Alignment::Left);
    let second_line =
        Line::styled(&testapp.second_sentence, Style::default().fg(Color::White))
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
    );
    frame.render_widget(
        Paragraph::new(format!("{}", testapp.max_time.as_secs() - testapp.start_time.elapsed().as_secs()))
        .block(
            Block::default()
            .borders(Borders::ALL).border_type(BorderType::Rounded)
            )
        ,
        // Block::new().title("protu").borders(Borders::ALL).border_type(BorderType::Rounded),
        Rect::new(width / 2 - (width / 3) - 5, height / 2 - 2, 5, 3)
        );
    
}

pub fn render_menu<B: Backend>(app: &mut App, frame: &mut Frame) {
    frame.render_widget(
        
        Paragraph::new(format!("Welcome to Olagem !\n{:?}", app.status))
        .block(
            Block::default()
                .title("olagem")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)),
        frame.size()
        )
}

pub fn render_resluts<B: Backend>(app: &mut App, frame: &mut Frame) {
    frame.render_widget(
        
        Paragraph::new(format!("Congratulation, you typed {} words in {:?} seconds !. This translate to {} WPM on the website 10FastFingers.\n
                               Press CTRL+r to restart.", app.results.typed, app.results.time.as_secs(), app.results.wpm))
        .block(
            Block::default()
                .title("olagem")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)),
        frame.size()
        )
}
