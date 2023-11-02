use std::{
    error::Error,
    io
};

use crossterm::{
    terminal::{ enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen},
    execute,
    event::{EnableMouseCapture, DisableMouseCapture}
};

use ratatui::{
    prelude::CrosstermBackend,
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // if let Err(err) = res {
        // println!("{err:?}");
    // }

    Ok(())
}
