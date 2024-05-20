use crate::app::{App, AppResult, AppStatus};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    backend::Backend,
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
            app.status = AppStatus::Menu;
        }
        _ => ()
    }
    Ok(())
}

pub fn render<B: Backend>(app: &mut App, frame: &mut Frame) {
    frame.render_widget(
        
        Paragraph::new(format!("{}", app.error))
        .block(
            Block::default()
                .title("olagem")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)),
        frame.size()
        )
}
