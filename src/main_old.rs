use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use core::str;
//use std::vec;
use clap::{App, Arg};

mod tests;
mod main;

fn check_input(pattern: &str, variable_chars: &str) -> bool {
    variable_chars.len() > pattern.len()
}

// fn is_valid_word(word: &str, exlude_chars: $str) -> bool {
fn is_valid_word(word: &str) -> bool {
    // Check if the word contains any numeric digits, '-', or '_'
    !word.chars().any(|c| c.is_digit(10) || c == '-' || c == '_' || c == '\'')
}

fn print_words_in_columns(words: Vec<String>, column_size: usize) {
    for (i, word) in words.iter().enumerate() {
        if i > 0 && i % column_size == 0 {
            println!(); // Move to the next line after `column_size` words
        }
        print!("{:<10}", word); // Print each word with some padding for better alignment
    }
    println!(); // Ensure the final line ends properly
    if words.len() > 10 {
        println!("To many words");
    }
}

fn read_wordlist(pattern_length: usize, file_path: &str, word_vec: &mut Vec<String>) -> Result<(), Error> {
    // Attempt to open the file
    let file = File::open(file_path)?;

    // Wrap the file handle in a BufReader
    let reader = BufReader::new(file);

    // Read each line from the file
    for line in reader.lines() {
        let word = line?;
        // Check if the length of the word matches pattern_length
        // and if the word contains no digits, '-' or '_'
        if word.len() == pattern_length && is_valid_word(&word) {
            word_vec.push(word.to_lowercase());
        }
    }
    Ok(())
}

fn findword(pattern: &str, variable_chars: &str, chars_not_exists: &str, wordlist: &mut Vec<String>) -> Vec<String> {
    if pattern.contains('*') {
        // Step 1: Filter words based on the pattern
        let matched_words: Vec<String> = wordlist.iter()
            .filter(|word| {
                word.chars().zip(pattern.chars()).all(|(w_char, p_char)| p_char == '*' || w_char == p_char)
            })
            .cloned()
            .collect();

        // Step 2: Remove words that contain chars in chars_not_exists
        let excl_chars_not_exists: Vec<String> = if !chars_not_exists.is_empty() {
            matched_words.clone().into_iter()
                .filter(|word| {
                    !chars_not_exists.chars().any(|c_char| word.contains(c_char))
                })
                .collect()
        } else {
            matched_words.clone()
        };

        // Step 3: Further filter matched words to check for presence of variable_chars
        let filterd_words: Vec<String> = if !variable_chars.is_empty() {
            excl_chars_not_exists.clone().into_iter()
               .filter(|word| {
                    variable_chars.chars().any(|v_char| word.contains(v_char))
                })
            .collect()
        } else {
            matched_words
        };

        // return list of possible words    
        filterd_words
    } else {
        wordlist.iter()
            .filter(|word| {
                word.contains(pattern) &&
                variable_chars.chars().any(|v_char| word.contains(v_char)) &&
                !chars_not_exists.chars().any(|c_char| word.contains(c_char))
            })
            .cloned()
            .collect()
    }
}

fn main() {
    //let input = "audio";
    //let target_substring = "uniek";
    let file_path = "wordlist.txt";
    let mut word_vec: Vec<String> = Vec::new();
    // let mut word_vec = vec![
    //     String::from("raden"),
    //     String::from("kruid"),
    //     String::from("pruik"),
    //     String::from("pruim"),
    //     String::from("audio")
    // ];

    // Define CLI options using clap
    let matches = App::new("WordFinder")
        .version("0.1.alpha")
        .about("Search for words in a wordlist")
        .arg(
            Arg::with_name("letter_in_correct_place")
                .help("Provide letters in the correct place\nUse '*' if you do not know the letter.")
                .required(true),
        )
        .arg(
            Arg::with_name("letter_not_in_correct_place")
                .short('x')
                .long("letter-x")
                .value_name("LETTER_X")
                .help("Provide letter not in the correct place")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("letter_not_in_word")
                .short('y')
                .long("letter-y")
                .value_name("LETTER_Y")
                .help("Provide letterss do not exist in the word")
                .takes_value(true),
        )
        .get_matches();

    // Extract the values of named arguments
    let pattern = matches.value_of("letter_in_correct_place").unwrap_or_default();
    let variable_chars = matches.value_of("letter_not_in_correct_place").unwrap_or_default();
    let chars_not_exists: &str = matches.value_of("letter_not_in_word").unwrap_or_default();
    
    // input_varable cannot contain more letters than input.
    if check_input(pattern, variable_chars) {
        println!("To many characters provided for the option x.");
        println!("You have provided letters {}, should not be longer than {}", variable_chars, pattern.len());
        // Exit the program here
        return;
    } 

    // Call the function to read the wordlist
    if let Err(e) = read_wordlist(pattern.len(), file_path, &mut word_vec) {
        eprintln!("Error reading file {}: {}", file_path, e);
    }

    //println!("Searching for '{pattern}' out of {} possible words.", word_vec.len());
    
    let matching_words = findword(pattern, variable_chars, chars_not_exists, &mut word_vec);
    print_words_in_columns(matching_words, 5);
    //println!("{:?}", matching_words);

}