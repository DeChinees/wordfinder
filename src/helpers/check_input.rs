// src/helpers/check_input.rs
pub fn check_input(pattern: &str, variable_chars: &str) -> bool {
    variable_chars.len() > pattern.len()
}
