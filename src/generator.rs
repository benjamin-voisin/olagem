use std::{fs,str};
use rand::{Rng, rngs::ThreadRng};
use std::path::PathBuf;

use crate::app::AppResult;

type WordList = Vec<String>;

#[derive(Debug)]
pub struct Generator {
    rng: ThreadRng,
    word_list : WordList,
}


impl Generator {
    pub fn new(language : &str) -> AppResult<Self> {
        let language_path = language_to_path(language)?;
        match read_path(language_path) {
            Ok(word_list) => {
                Ok(
                    Self {
                        rng: rand::thread_rng(),
                        word_list,
                    }
                    )
            }
            Err(err) => Err(err),
        }
    }

    pub fn generate_one_line(&mut self, max_length : u16) -> AppResult<String>{
        let mut words = String::new();
        let mut word : &str;

        loop {
            word = &self.word_list[self.rng.gen_range(0..self.word_list.len())];
            if words.chars().count() + word.chars().count() + 2 > max_length.into() {
                break
            }
            words.push_str(word);
            words.push(' ');
        }

        Ok(words)
    }
}

fn language_to_path(language : &str) -> AppResult<Box<PathBuf>> {
    match dirs::config_dir() {
        Some(d) => Ok(Box::new(d.join("olagem").join("language").join(language))),
        None => Err(Box::from("No config directory found")),
    }
}

fn read_path(path : Box<PathBuf> ) -> AppResult<WordList> {
    let contents: Vec<u8> = fs::read(*path).expect("The config file for the specified language does not exists in ~/.config/olagem/language");

    let contents_string = str::from_utf8(&contents)?.to_string();

    let word_vector : Vec<String> = (contents_string).split('\n').map(|w| w.to_string()).collect();
    Ok(word_vector)
}
