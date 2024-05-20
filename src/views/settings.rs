use crate::app::{App, AppResult};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_event(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        }

        KeyCode::Esc => {
            app.quit();
        }

        _ => ()
    }
    Ok(())
}
