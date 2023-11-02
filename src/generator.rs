use std::fs;

use crate::app::AppResult;

type WordList = Vec<String>;

fn language_to_path(language : &str) -> String {
    format!("/usr/share/olagem/language/{}", language)
}

fn read_language(language : &str) -> AppResult<WordList> {
    let path = language_to_path(language);
    let contents = fs::read_to_string(path)?;

    let word_vector : Vec<String> = contents.split('\n').map(|w| w.to_string()).collect();
    // let word_list : Vec<String> = word_vector.iter().map(|w| w.to_string()).collect();
    // Ok(word_vector)
    Ok(word_vector)
    // Ok(Vec::new())
}

pub fn generate_one_line() -> AppResult<String>{
    let word_list = read_language("french")?;
    Ok(format!("{}, {}, {}", word_list[0], word_list[1], word_list[2]))
    // "protu".to_string()
}
