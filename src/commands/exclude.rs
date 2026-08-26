// src/commands/exclude.rs
#[allow(dead_code)]
pub fn exclude(word_vec: &mut Vec<String>, word: &str) {
    if let Some(pos) = word_vec.iter().position(|x| x == word) {
        word_vec.remove(pos);
        println!("Word '{}' excluded.", word);
    } else {
        println!("Word '{}' not found in the list.", word);
    }
}
