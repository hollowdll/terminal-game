pub fn extract_first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
