use regex::Regex;

pub fn generate_word_patterns() -> [Regex; 1] {
    [
        regex::Regex::new(r"\$(\w+[\w\d_\-]*)*").unwrap()
    ]
}