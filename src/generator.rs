use std::fs;
use rand::{Rng, rngs::ThreadRng};

use crate::app::AppResult;

type WordList = Vec<String>;

#[derive(Debug)]
pub struct Generator {
    rng: ThreadRng,
    word_list : WordList,
    language : String,
    path : String,
}


impl Generator {
    pub fn new(language : String) -> Self {
        Self {
            rng : rand::thread_rng(),
            language : language.clone(),
            path : language_to_path(&language.clone()),
            word_list : read_path(&language_to_path(&language)).unwrap(),
        }
    }

    pub fn generate_one_line(&mut self, max_length : usize) -> AppResult<String>{
        let mut words = String::new();
        let mut word : &str;

        loop {
            word = &self.word_list[self.rng.gen_range(0..self.word_list.len())];
            if words.chars().count() + word.chars().count() > max_length {
                break
            }
            words.push_str(word);
            words.push(' ');
        }

        Ok(words)
    }
}

fn language_to_path(language : &str) -> String {
    format!("/usr/share/olagem/language/{}", language)
}

fn read_path(path : &str) -> AppResult<WordList> {
    let contents = fs::read_to_string(path)?;

    let word_vector : Vec<String> = contents.split('\n').map(|w| w.to_string()).collect();
    Ok(word_vector)
}