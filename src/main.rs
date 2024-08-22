// src/main.rs
mod commands; // Reference the module in the commands directory
mod helpers;

use std::io::{self, Write};

fn main() {
    let mut word_vec: Vec<String> = Vec::new(); // Initialize an empty Vec<String>
    let mut include_chars: Option<String> = None;
    let mut exclude_chars: Option<String> = None;
    let mut word_length: usize = 0;
    commands::init_dictionary(&mut word_vec);

    loop {
        print!("Enter command: ");
        io::stdout().flush().unwrap(); // Flush the output to display prompt

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Parse command and argument
        let mut parts = input.split_whitespace();
        let command = parts.next();
        let argument = parts.next();

        match command {
            Some("help") => {
                commands::help();
            }
            Some("find") => {
                if let Some(word) = argument {
                    if word_length == 0 {
                        // This is the first time we do a search.
                        // We filter out only words that not equal in length to word_len().
                        word_length = word.len();
                        commands::set_dictionary(word.len(), &mut word_vec)
                    }

                    // Check word_length
                    if let Err(error) = helpers::check_word_length(word_length, word.len()) {
                        commands::help_word_length(word, &include_chars, &exclude_chars);
                        return; // Exit the clause early
                    }

                    // Capture the result of the `find` function
                    let filtered_words = commands::find(word, include_chars.clone(), exclude_chars.clone(), &mut word_vec);

                    // Print or use the filtered words
                    if filtered_words.is_empty() {
                        println!("No words found matching the criteria.");
                    } else {
                        println!("Found words: {:?}", filtered_words);
                    }
                } else {
                    println!("Usage: find <word>");
                }
            }
            Some("include") => {
                if let Some(word) = argument {
                    // Check if the argument is "help"
                    if word == "help" {
                        println!("Usage: include <word>");
                        println!("  <word> - the characters to include in the search");
                        println!("If no argument is given, prints the current value of include_chars.");
                    } else {
                        // Add new characters to include_chars
                        if let Some(ref mut existing_chars) = include_chars {
                            // Append the new characters to the existing ones
                            existing_chars.push_str(&word);
                        } else {
                            // Initialize include_chars with the new characters
                            include_chars = Some(word.to_owned());
                        }
                        println!("Updated include_chars to: {}", include_chars.as_ref().unwrap());
                    }
                } else {
                    // No argument provided, print the current value of include_chars
                    match &include_chars {
                        Some(chars) => println!("Current include_chars: {}", chars),
                        None => println!("No include_chars set."),
                    }
                }
            }
            Some("exclude") => {
                if let Some(word) = argument {
                    // Check if the argument is "help"
                    if word == "help" {
                        println!("Usage: exclude <word>");
                        println!("  <word> - the characters to include in the search");
                        println!("If no argument is given, prints the current value of include_chars.");
                    } else {
                        // Add new characters to include_chars
                        if let Some(ref mut existing_chars) = exclude_chars {
                            // Append the new characters to the existing ones
                            existing_chars.push_str(&word);
                        } else {
                            // Initialize include_chars with the new characters
                            exclude_chars = Some(word.to_owned());
                        }
                        println!("Updated exclude_chars to: {}", exclude_chars.as_ref().unwrap());
                    }
                } else {
                    // No argument provided, print the current value of exclude_chars
                    match &exclude_chars {
                        Some(chars) => println!("Current exclude_chars: {}", chars),
                        None => println!("No exclude_chars set."),
                    }
                }
            }
            Some("list") => {
                if let Some(arg) = argument {
                    helpers::print_words_in_columns(&mut word_vec, arg, 6)
                } else {
                    helpers::print_words_in_columns(&mut word_vec, "", 6)
                }
            }
            Some("reset") => {
                if let Some(word) = argument {
                    // Check if the argument is "help"
                    if word == "include" {
                        include_chars = None;
                        println!("Reset include_chars to None");
                    }
                    if word == "exclude" {
                        exclude_chars = None;
                        println!("Reset exclude_chars to None");
                    }
                } else {
                    commands::init_dictionary(&mut word_vec);
                    include_chars = None;
                    exclude_chars = None;
                }
            }
            Some("exit") => {
                println!("Exiting program...");
                break;
            }
            _ => {
                println!("Unknown command. Type 'help' to see the list of available commands.");
            }
        }
    }
}