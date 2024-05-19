use crate::app::{App, AppResult, AppStatus};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

fn panic_handle_key_event(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
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
        _ => ()
    }
    Ok(())
}


fn menu_handle_key_event(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
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
        _ => ()
    }
    Ok(())
}

fn test_handle_key_event(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC`
        KeyCode::Esc  => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
            else { 
                match &mut app.testapp {
                    None => (),
                    Some(testapp) => testapp.add_ch('c')?
                }
            }
        }

        KeyCode::Backspace => {
            match &mut app.testapp {
                None => (),
                Some(testapp) =>  {testapp.delete_ch();},
            }
        }
        
        // Check for CTRL+BACKSPACE (for some reason it is not supported the same way as CTRL+C
        KeyCode::Char('h') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
            match &mut app.testapp {
                None => (),
                Some(testapp) => testapp.delete_word()?
            }
        }
        
        KeyCode::Char(c) => {
            match &mut app.testapp {
                None => (),
                Some(testapp) => testapp.add_ch(c)?
            }
        }

        _ => {}
    }
    Ok(())
}

fn result_handle_key_event(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
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

fn settings_handle_key_event(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
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

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.status {
        AppStatus::Menu => menu_handle_key_event(key_event, app)?,
        AppStatus::Test => test_handle_key_event(key_event, app)?,
        AppStatus::Results => result_handle_key_event(key_event, app)?,
        AppStatus::Settings => settings_handle_key_event(key_event, app)?,
        AppStatus::Panic => panic_handle_key_event(key_event, app)?,
    }
    Ok(())
}
