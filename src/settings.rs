use std::time::Duration;


#[derive(Debug)]
pub struct Settings {
    pub max_length: usize,
    pub language: String,
    pub max_time : Duration,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            max_length: 80,
            language: "french".to_string(),
            max_time: Duration::from_secs(60),
        }
    }
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_language(&mut self, language: &str) {
        self.language = language.to_string();
    }

    pub fn set_max_length(&mut self, max_length: usize) {
        self.max_length = max_length;
    }

    pub fn set_max_duration(&mut self, max_time: usize) {
        self.max_time = Duration::from_secs(max_time as u64);
    }
}
