// src/commands/include.rs
pub fn include(word_vec: &mut Vec<String>, word: &str) {
    if !word_vec.contains(&word.to_string()) {
        word_vec.push(word.to_string());
        println!("Word '{}' included.", word);
    } else {
        println!("Word '{}' is already in the list.", word);
    }
}