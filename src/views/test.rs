use crate::app::{App, AppResult};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, prelude::Rect,
    prelude::{Span, Line},
};

pub fn handle_key_event(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
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

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame) -> AppResult<()> {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let testapp_opt = app.testapp.as_ref();
    match testapp_opt {
        Some(testapp) => {
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
                .style(Style::default().fg(Color::White))
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
                Rect::new(width / 2 - (width / 3) - 5, height / 2 - 2, 5, 3)
                );
            Ok(())
        }
        None => Err(Box::from("Unable to take testapp as ref")),
    }

}
