// src/main.rs
mod commands; // Reference the module in the commands directory
mod helpers;

use std::io::{self, Write};

fn main() {
    let mut word_vec: Vec<String> = Vec::new(); // Initialize an empty Vec<String>
    commands::reset_words(&mut word_vec);

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
                    commands::find(&word_vec, word);
                } else {
                    println!("Usage: find <word>");
                }
            }
            Some("include") => {
                if let Some(word) = argument {
                    commands::include(&mut word_vec, word);
                } else {
                    println!("Usage: include <word>");
                }
            }
            Some("exclude") => {
                if let Some(word) = argument {
                    commands::exclude(&mut word_vec, word);
                } else {
                    println!("Usage: exclude <word>");
                }
            }
            Some("list") => {
                if let Some(arg) = argument {
                    helpers::print_words_in_columns(&mut word_vec, arg, 6)
                } else {
                    helpers::print_words_in_columns(&mut word_vec, "", 6)
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