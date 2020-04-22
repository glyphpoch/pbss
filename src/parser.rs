use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

pub fn read_file(file: String) -> String {
	let mut result = String::new().to_owned();
	let mut file = File::open(file).expect("Can't open file");
	let mut contents = String::new();
	file.read_to_string(&mut contents).expect("Can't read file");

	let re = Regex::new(r".+\n?\s?\{(\n?\t?.+: .+\n)+}").unwrap();

	for block in re.captures_iter(&contents){
		let new_text: String = block[0].replace("\t", "").to_string();
		result.push_str(&new_text);
		result.push_str("\n");
	}
	return result;
}

pub fn break_blocks(block: String) -> Vec<String> {
	let items = block.split("}");
	let items: Vec<&str> = items.collect();
	let items: Vec<&str> = items[..(items.len() -1)].to_vec();
	let mut blocks: Vec<String> = [].to_vec();
	for item in &items {
		let mut item: String = item.to_string();
		item.push('}');
		blocks.push(item)
	}
	return blocks;
}

pub fn strip_newlines(block: &mut String) {
	*block = block.replace("\n", "");
}

pub fn break_tokens(block: &mut String) -> (&str, &str) {
	let place = block.find("{");
	match place {
		Some(p) => {
			if block.chars().nth(p - 1).unwrap() == ' ' {
				block.remove(p - 1);
			};
		},
		None => (),
	};
	let place = block.find("{");
	let tokens = block.split_at(place.unwrap());
	
	return tokens;
}