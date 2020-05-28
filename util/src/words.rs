use regex::Regex;

pub fn generate_word_patterns() -> [Regex; 2] {
    [
        Regex::new(r"\$(\w+[\w\d_\-]*)*").unwrap(),
        Regex::new(r"(\d+)\s*\t*([+\-*/]{1})\s*\t*(\d+)").unwrap(),
    ]
}
