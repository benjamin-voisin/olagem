use olagem::{
    app::{App, AppResult, AppStatus},
    event::{Event, EventHandler},
    views,
    tui::Tui,
    config,
};

use std::{env, io, process::exit};

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

fn parcour_arguments() {
    let args: Vec<String> = env::args().collect();
    for arg in args.iter() {
        match arg.as_str() {
            "-h" => {
                println!("olagem, a typing-speed test");
                exit(0)
            },
            "-v" => {
                println!("olagem v0.1.2");
                exit(0)
            }
            _ => (),
        }
    }
}

fn main() -> AppResult<()> {

    // Check if config dir exists
    if !config::exist_config_dir()? {
        // Copy files from /usr/share/olagem to CONFIG_DIR/olagem
        config::copy_to_config_dir()?;
    };
    // Create an application.

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let width = terminal.size()?.width;


    let mut app = App::new();

    parcour_arguments();

    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    app.settings.set_max_length(((width / 3) * 2).into());
    // Start the main loop.
    while app.running {
        // Render the user interface.
        match tui.draw(&mut app) {
            Ok(()) => (),
            Err(err) => app.panic(err),
        }
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => {
                match app.status {
                    AppStatus::Menu => views::menu::handle_key_event(key_event, &mut app)?,
                    AppStatus::Test => views::test::handle_key_event(key_event, &mut app)?,
                    AppStatus::Results => views::results::handle_key_event(key_event, &mut app)?,
                    AppStatus::Settings => views::settings::handle_key_event(key_event, &mut app)?,
                    AppStatus::Panic => views::panic::handle_key_event(key_event, &mut app)?,
                }
            },
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
