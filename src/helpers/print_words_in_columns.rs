use std::io;

pub fn print_words_in_columns(word_vec: &Vec<String>, argument: &str, column_size: usize) {
    if argument == "all" || (word_vec.len() <= 10 && argument.is_empty()) {
        print_words(word_vec, column_size);
    } else {
        println!("Too many words");
    }
}

fn print_words(word_vec: &Vec<String>, column_size: usize) {
    for (i, word) in word_vec.iter().enumerate() {
        if i > 0 && i % column_size == 0 {
            println!(); // Move to the next line after `column_size` words
        }
        print!("{:<10}", word); // Print each word with some padding for better alignment
    }
    println!(); // Ensure the final line ends properly
}