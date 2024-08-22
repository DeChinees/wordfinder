
pub fn find(pattern: &str,
            include_chars: Option<String>,
            exclude_chars: Option<String>,
            word_vec: &mut Vec<String>) -> Vec<String> {
    // Unwrap include_chars and exclude_chars, using an empty string as the default.
    let allowed_chars = include_chars.unwrap_or_default();
    let forbidden_chars = exclude_chars.unwrap_or_default();

    if pattern.contains('*') {
        // Step 1: Filter words that match the wildcard pattern.
        let matched_words: Vec<String> = word_vec.iter()
            .filter(|word| {
                word.chars()
                    .zip(pattern.chars())
                    .all(|(word_char, pattern_char)| {
                        pattern_char == '*' || word_char == pattern_char
                    })
            })
            .cloned()
            .collect();

        // Step 2: Further filter matched words based on allowed and forbidden characters.
        matched_words.into_iter()
            .filter(|word| {
                let include_check = allowed_chars.is_empty() ||
                    allowed_chars.chars().any(|include_char| word.contains(include_char));

                let exclude_check = forbidden_chars.is_empty() ||
                    !forbidden_chars.chars().any(|forbidden_char| word.contains(forbidden_char));

                include_check && exclude_check
            })
            .collect()
    } else {
        // Directly filter word_vec if the pattern does not contain a wildcard.
        word_vec.iter()
            .filter(|word| {
                let include_check = allowed_chars.is_empty() ||
                    allowed_chars.chars().any(|include_char| word.contains(include_char));

                let exclude_check = forbidden_chars.is_empty() ||
                    !forbidden_chars.chars().any(|forbidden_char| word.contains(forbidden_char));

                word.contains(pattern) && include_check && exclude_check
            })
            .cloned()
            .collect()
    }
}