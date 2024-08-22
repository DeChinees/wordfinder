pub fn check_word_length(word_length: usize, word_len: usize) -> Result<(), String> {
    if word_length != word_len {
        return Err("Word length does not match the expected length.".to_string());
    }
    Ok(())
}
pub fn check_chars_length(chars: &Option<String>, word_len: usize) -> Result<(), String> {
    if let Some(ref chars_str) = chars {
        if chars_str.len() != word_len {
            return Err("include_chars length does not match word length.".to_string());
        }
    }
    Ok(())
}