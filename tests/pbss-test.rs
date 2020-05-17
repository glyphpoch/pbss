use pbss::generate_basic_patterns;
use pbss::parser::compile;
use std::fs::File;
use std::io::prelude::*;

#[test]
fn test_pbss() {
    let mut result = String::new();
    let mut file = File::open("tests/result.css").unwrap();
    file.read_to_string(&mut result);
    let patterns = generate_basic_patterns();
    let contents = compile(&"tests/test.pbss".to_string(), &patterns);
    assert_eq!(contents, result);
}
