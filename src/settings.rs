use std::time::Duration;

use std::fs;
use std::error;

use serde::Deserialize;


#[derive(Debug)]
pub struct Settings {
    pub max_length: u16,
    pub language: String,
    pub max_time : Duration,
}

#[derive(Deserialize, Debug)]
struct Config {
    defaults: Defaults,
}

#[derive(Deserialize, Debug)]
struct Defaults {
    language: String,
    time: u64,
}

fn try_read_settings_from_config() -> Result<Settings, Box<dyn error::Error>> {
    // We can use unwrap because the config dir is at least made a startup
    let config_file = dirs::config_dir()
        .unwrap()
        .join("olagem/config.toml");
    let config_string = fs::read_to_string(&config_file)?;


    let config: Config = toml::from_str(&config_string).unwrap();
    Ok(Settings {
        language: config.defaults.language,
        max_time: Duration::new(config.defaults.time,0),
        max_length: 80,

    })
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            max_length: 80,
            language: "english".to_string(),
            max_time: Duration::from_secs(60),
        }
    }
}

impl Settings {
    pub fn new() -> Self {
        match try_read_settings_from_config() {
            Ok(settings) => settings,
            Err(_) => Self::default(),
        }
    }

    pub fn set_language(&mut self, language: &str) {
        self.language = language.to_string();
    }

    pub fn set_max_length(&mut self, max_length: u16) {
        self.max_length = max_length;
    }

    pub fn set_max_duration(&mut self, max_time: u64) {
        self.max_time = Duration::from_secs(max_time);
    }
}

