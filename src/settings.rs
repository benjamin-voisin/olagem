
#[derive(Debug)]
pub struct Settings {
    pub max_length: usize,
    pub language: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            max_length: 80,
            language: "french".to_string(),
        }
    }
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }
}
