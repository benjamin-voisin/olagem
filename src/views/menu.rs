use crate::app::{App, AppResult};
use crate::stats::render_chart;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    backend::Backend,
    prelude::{Constraint, Layout, Direction},
    layout::Alignment,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};


pub fn handle_key_event(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc => {
            app.quit();
        },

        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }

        KeyCode::Char('s') => {
            app.start_test().unwrap_or_else(|err| app.panic(err));
        }

        KeyCode::Char('m') => {
            app.go_to_settings().unwrap_or_else(|err| app.panic(err));
        }
        _ => ()
    }
    Ok(())
}


pub fn render<B: Backend>(app: &mut App, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(frame.size());

    // let chart = create_chart();
    render_chart(layout[1], frame);
    // frame.render_widget(chart, layout[1]);

    frame.render_widget(
        
        Paragraph::new(format!("Welcome to Olagem !\n{:?}", app.status))
        .block(
            Block::default()
                .title("olagem")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)),
        layout[0]
        );
}
