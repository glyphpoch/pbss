use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

pub fn read_file(file: &String) -> String{
	let mut result = String::new().to_owned();
	let mut file = File::open(file).expect("Can't open file");
	let mut contents = String::new();
	file.read_to_string(&mut contents).expect("Can't read file");

	return contents;
}

pub fn parse(contents: String){
	let parsed_string: String = String::new();
	let patterns: Vec<Regex> = vec![
		Regex::new(r"\$([\w\d_-]*) *\t*: *\t*([\w\d\(\) \t!,(//)]*);").unwrap(),
		Regex::new(r"//(.*)").unwrap(),
		Regex::new(r"([\w\- \.:\d]*) *\t*\{").unwrap(),
		Regex::new(r"\s*\t*([\w\d \-]*) *\t*: ([\w\d\(\) \t!,(//)]*);").unwrap(),
		Regex::new(r"\}").unwrap(),
		Regex::new(r"@\w+([\w\-\d \(\):]*) *\t*\{").unwrap(),
	];

	for (line) in contents.lines(){
		for pattern in &patterns{
			if pattern.is_match(line) {; break;};}
	}
}