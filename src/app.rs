use std::error;

use std::fs::OpenOptions;
use std::io::prelude::*;

use crate::{
    results::Results, settings::Settings, testapp::{self, TestApp}
};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum AppStatus {
    Menu,
    Settings,
    Test,
    Results,
    Panic,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    pub status: AppStatus,
    pub testapp: Option<TestApp>,
    pub settings: Settings,
    pub results: Results,
    pub error: Box<dyn error::Error>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            running: true,
            status: AppStatus::Menu,
            testapp: None,
            settings: Settings::new(),
            results: Results::new(),
            error: Box::from("No errors"),
        }
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
                if testapp.start_time.elapsed() > testapp.max_time { self.stop_test() }
            
        }
    }

    pub fn panic(&mut self, error: Box<dyn error::Error>) -> () {
        self.error = error;
        self.status = AppStatus::Panic;
    }

    pub fn start_test(&mut self) -> AppResult<()>{
        self.testapp = Some(TestApp::new(&self.settings.language, self.settings.max_length, self.settings.max_time)?);
        self.status = AppStatus::Test;
        Ok(())
    }

    pub fn go_to_settings(&mut self) -> AppResult<()> {
        self.status = AppStatus::Settings;
        Ok(())
    }

    fn stop_test(&mut self) {
        self.status = AppStatus::Results;
        let testapp_opt = self.testapp.as_ref();
        match testapp_opt {
            Some(testapp) => {
                // Write results as csv to a file
                let mut result_file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(
                        dirs::config_dir().unwrap().join("olagem/results.csv")
                    )
                    .unwrap();

                writeln!(result_file, "{};{:?};{}",
                    self.settings.language,
                    testapp.start_time.elapsed(),
                    testapp.total_typed
                ).unwrap();
                self.results.typed = testapp.total_typed;
                self.results.time = testapp.start_time.elapsed();
                self.testapp = None;
                self.results.set_wpm();
            },
            None => self.panic(Box::from("Unable to convert testapp to ref")),
        }
    }
}
