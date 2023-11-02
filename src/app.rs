use std::{error, ops::Add};
use crate::generator::Generator;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// generator
    pub generator: Generator,
    // pub first_sentence: String,
    //
    pub max_length: usize,

    pub correctly_typed: String,
    pub wrongly_typed: String,
    pub to_type: String,

    pub second_sentence : String,
    /// current state
    pub cursor_position : usize,
    // pub typed_text : String,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(language : &str, max_length: usize) -> AppResult<Self> {
        let mut generator = Generator::new(language);
        let first_sentence = generator.generate_one_line(max_length)?;
        let second_sentence = generator.generate_one_line(max_length)?;
        Ok (Self {
            running: true,
            max_length,
            generator,
            // first_sentence,
            correctly_typed: String::new(),
            wrongly_typed: String::new(),
            to_type: first_sentence,
            second_sentence,
            cursor_position : 0,
            // typed_text : String::new(),
        })
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn add_ch(&mut self, c: char) {
        if c == self.to_type.chars().nth(0).unwrap() {
            self.to_type.remove(0);
            self.correctly_typed.push(c);
        }
        else {
            self.wrongly_typed.push(c);
        }
        if self.to_type.len() == 0 {
            self.to_type = self.second_sentence.to_owned();
            self.correctly_typed = String::new();
            self.wrongly_typed = String::new();
            self.second_sentence = self.generator.generate_one_line(self.max_length).unwrap();
        }
        self.cursor_position += 1;
    }

    pub fn delete_ch(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            if self.wrongly_typed.len() > 0 {
                let wrongly_typed_length = self.wrongly_typed.chars().count();
                self.wrongly_typed = self.wrongly_typed.chars().take(wrongly_typed_length - 1).collect();
            }
            else {
                let correctly_typed_length = self.correctly_typed.chars().count();
                self.correctly_typed = self.correctly_typed.chars().take(correctly_typed_length - 1).collect();
            }
        }
    }
}
