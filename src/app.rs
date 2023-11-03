use std::error;

use crate::{
    testapp::TestApp,
    settings::Settings,
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum AppStatus {
    Menu,
    Settings,
    Test,
    Results,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    pub status: AppStatus,
    pub testapp: Option<TestApp>,
    pub settings: Settings,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> AppResult<Self> {
        Ok (Self {
            running: true,
            status: AppStatus::Menu,
            testapp: None,
            settings: Settings::new(),
        })
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        match &mut self.testapp {
            None => (),
            Some(testapp) =>
                if testapp.start_time.elapsed().as_secs() > testapp.max_time.as_secs() {testapp.running = false}
            
        }
    }

    pub fn start_test(&mut self) -> AppResult<()>{
        self.testapp = Some(TestApp::new(&self.settings.language, self.settings.max_length)?);
        self.status = AppStatus::Test;
        Ok(())
    }
}
