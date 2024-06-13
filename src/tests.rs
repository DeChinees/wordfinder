// src/tests.rs

#[cfg(test)]
mod tests {
    use crate::findword;
    use crate::print_words_in_columns;
    #[test]
    fn test_findword_with_pattern_and_variable_chars_and_chars_not_exists() {
        let mut word_vec = vec![
            String::from("raden"),
            String::from("kruid"),
            String::from("pruik"),
            String::from("pruim"),
            String::from("audio"),
            String::from("unique"),
            String::from("united"),
            String::from("unity"),
            String::from("union"),
            String::from("units"),
        ];

        let result = findword("**ui*", "p", "a", &mut word_vec);
        assert_eq!(result, vec!["pruik", "pruim"]);
    }

    #[test]
    fn test_findword_with_pattern_no_extra_options() {
        let mut word_vec = vec![
            String::from("raden"),
            String::from("kruid"),
            String::from("pruik"),
            String::from("pruim"),
            String::from("audio"),
            String::from("unique"),
            String::from("united"),
            String::from("unity"),
            String::from("union"),
            String::from("units"),
        ];

        let result = findword("**d**", "", "", &mut word_vec);
        assert_eq!(result, vec!["raden", "audio"]);
    }

    #[test]
    fn test_findword_without_pattern() {
        let mut word_vec = vec![
            String::from("raden"),
            String::from("kruid"),
            String::from("pruik"),
            String::from("pruim"),
            String::from("audio"),
            String::from("unique"),
            String::from("united"),
            String::from("unity"),
            String::from("union"),
            String::from("units"),
        ];

        let result = findword("ni", "", "qs", &mut word_vec);
        assert_eq!(result, vec!["united", "unity", "union"]);
    }


    #[test]
    fn test_print_words_in_columns() {
        let words = vec![
            String::from("one"),
            String::from("two"),
            String::from("three"),
            String::from("four"),
            String::from("five"),
            String::from("six"),
            String::from("seven"),
            String::from("eight"),
            String::from("nine"),
            String::from("ten"),
        ];

        print_words_in_columns(words, 5);
        // Visual inspection required for this test as it prints to the console.
    }
}
