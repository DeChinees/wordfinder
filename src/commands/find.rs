// src/commands/find.rs
pub fn find(word_vec: &Vec<String>, word: &str) {
    if word_vec.contains(&word.to_string()) {
        println!("Word '{}' found in the list.", word);
    } else {
        println!("Word '{}' not found in the list.", word);
    }
}