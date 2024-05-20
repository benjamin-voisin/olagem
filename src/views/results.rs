use crate::app::{App, AppResult};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    backend::Backend,
    layout::Alignment,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn handle_key_event(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Char('r') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.start_test()?;
            }
        },

        KeyCode::Esc => {
            app.quit();
        }

        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }
        _ => ()
    }
    Ok(())
}

pub fn render<B: Backend>(app: &mut App, frame: &mut Frame) {
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
