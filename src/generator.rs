use std::{fs,str};
use rand::{Rng, rngs::ThreadRng};
use std::path::PathBuf;

use crate::app::AppResult;

type WordList = Vec<String>;

#[derive(Debug)]
pub struct Generator {
    rng: ThreadRng,
    word_list : WordList,
    // language : String,
    // path : String,
}


impl Generator {
    pub fn new(language : &str) -> Self {
        Self {
            rng : rand::thread_rng(),
            // language : language.to_string(),
            // path : language_to_path(language),
    // let install_path = dirs::config_dir()
        // .expect("Couldn't find a configuration directory to install to.")
        // .join("olagem");
            word_list : read_path(language_to_path(language)).unwrap(),
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

fn language_to_path(language : &str) -> Box<PathBuf> {
    Box::new(dirs::config_dir()
                .expect("Couldn't find the configuration file, supposedly at ~/.config/olagem.")
                .join("olagem")
                .join("language")
                .join(language))
}

fn read_path(path : Box<PathBuf> ) -> AppResult<WordList> {
    // panic!("{:?}", path);
    let contents: Vec<u8> = fs::read(*path).expect("The config file for the specified language does not exists in ~/.config/olagem/language");

    let contents_string = str::from_utf8(&contents).unwrap().to_string();

    let word_vector : Vec<String> = (contents_string).split('\n').map(|w| w.to_string()).collect();
    Ok(word_vector)
}
