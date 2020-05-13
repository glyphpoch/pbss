use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

pub fn read_file(file: &String) -> String {
    let mut file = File::open(file).expect("Can't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Can't read file");

    return contents;
}
