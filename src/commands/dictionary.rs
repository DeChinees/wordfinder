use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn init_dictionary(word_vec: &mut Vec<String>) {
    match init(word_vec) {
        Ok(()) => {
            println!("Dictionary initialized successfully.");
        }
        Err(e) => {
            eprintln!("Failed to initialize dictionary: {}", e);
            // Handle the error, e.g., by exiting the function early
        }
    }
}

pub fn set_dictionary(word_length: usize, word_vec: &mut Vec<String>) {
    // Retain only the words that match the specified word_length
    word_vec.retain(|word| word.len() == word_length);
}

fn is_valid_word(word: &str) -> bool {
    // Check if the word contains any numeric digits, '-', or '_'
    !word.chars().any(|c| c.is_digit(10) || c == '-' || c == '_' || c == '\'')
}

fn init(word_vec: &mut Vec<String>) -> io::Result<()> {
    // Clear the current contents of word_vec
    word_vec.clear();

    // Define the path to the wordlist file
    let path = Path::new("src/dictionary/wordlist.txt");

    // Open the file in read-only mode
    let file = File::open(&path)?;

    // Use a buffered reader to read the file line by line
    let reader = io::BufReader::new(file);

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line?; // Handle potential errors
        if is_valid_word(&line) {
            word_vec.push(line.to_lowercase());
        }
    }

    println!("Word list reset with {} words.", word_vec.len());
    Ok(())
}