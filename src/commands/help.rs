pub fn help() {
    println!("Available commands:");
    println!("  help                        - Show this help message");
    println!("  find <word-pattern>         - Find the word in the list");
    println!("  include <character-pattern> - Add the word to the list");
    println!("  exclude <character-pattern> - Remove the word from the list");
    println!("  exit                        - Exit the program");
}
pub fn help_word_length(word: &str, include_chars: &Option<String>, exclude_chars: &Option<String>) {
    println!("The length pattern does not match.");
    println!("  Searching for {word}");

    // Use pattern matching to handle the Option<String> variables
    match include_chars {
        Some(chars) => println!("  Including {}", chars),
        None => println!("  Including none"),
    }

    match exclude_chars {
        Some(chars) => println!("  Excluding {}", chars),
        None => println!("  Excluding none"),
    }
}