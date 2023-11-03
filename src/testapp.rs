use std::{
    error,
    time::{Instant, Duration}
};
use crate::generator::Generator;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum AppStatus {
    Menu,
    Settings,
    Test,
    Results,
}

/// The struct for the Test part

#[derive(Debug)]
pub struct TestApp {
    pub running: bool,
    pub generator: Generator,
    pub max_length: usize,
    pub correctly_typed: String,
    pub wrongly_typed: String,
    pub to_type: String,

    pub second_sentence : String,
    pub cursor_position : usize,

    pub start_time: Instant,
    pub max_time: Duration,

    pub total_typed: usize,
}

impl TestApp {
    pub fn new(language : &str, max_length: usize, max_time: Duration) -> AppResult<Self> {
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

            start_time: Instant::now(),
            max_time,
            total_typed: 0,
        })
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        if self.start_time.elapsed().as_secs() > self.max_time.as_secs() {
            self.running = false;
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn add_ch(&mut self, c: char) {
        if c == self.to_type.chars().nth(0).unwrap() && self.wrongly_typed.len() == 0 {
            self.to_type.remove(0);
            self.correctly_typed.push(c);
            self.total_typed += 1;
        }
        else {
            // On met dans wrongly_typed le premier élément de to_type, qu'on enlève
            // let car = self.to_type.chars().nth(0).unwrap();
            let mut to_type_iter = self.to_type.chars();
            let car = to_type_iter.next().unwrap();
            self.wrongly_typed.push(car);
            self.to_type = to_type_iter.collect();

            // self.to_type = self.to_type.chars().next();
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
                // let mut wrongly_typed_iter = self.wrongly_typed.chars();
                let wrongly_typed_length = self.wrongly_typed.chars().count();
                let car = self.wrongly_typed.chars().last().unwrap();
                self.to_type = String::from(car) + &self.to_type;
                // self.wrongly_typed = wrongly_typed_iter.collect();
                self.wrongly_typed = self.wrongly_typed.chars().take(wrongly_typed_length - 1).collect();
            }
            else {
                // let correctly_typed_iter = self.correctly_typed.chars();
                let correctly_typed_length = self.correctly_typed.chars().count();
                let last_ch = self.correctly_typed.chars().last().unwrap();
                self.correctly_typed = self.correctly_typed.chars().take(correctly_typed_length - 1).collect();
                self.to_type = last_ch.to_string() + &self.to_type;
                self.total_typed -= 1;

            }
        }
    }

}
