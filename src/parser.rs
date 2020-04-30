use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashMap;

pub fn read_file(file: &String) -> String{
	let mut file = File::open(file).expect("Can't open file");
	let mut contents = String::new();
	file.read_to_string(&mut contents).expect("Can't read file");

	return contents;
}

pub fn strip_comments(contents: String) -> String{
	let comment = Regex::new(r#"/\*[\d\w \n\t!@#$%^&*\(\)_\-\+=~`|\\\{\}\[\]'":;<>,./?]*\\*/"#).unwrap();
	let text = comment.replace_all(&contents, "");
	return text.to_string()
}

pub fn strip_empty_lines(string: String) -> String{
	let newline = Regex::new(r"^ *\n*$").unwrap();
	let mut raw_string = String::new();
	for mut line in string.lines(){
		let edit_line = &newline.replace_all(line, "");
		if ! edit_line.is_empty() {
			raw_string.push_str(edit_line);
			raw_string.push_str("\n");
		}
	}
	return raw_string;
}

pub fn track_variables(string: String)-> (HashMap<String, String>, String)
 {
	let variable = Regex::new(r"\$(\w+[\w\d_\-]*) *\t*: *\t*([\d\w_\-\(\),]*);")
		.unwrap();
	let mut variable_index: HashMap<String, String> = HashMap::new();
	for cap in variable.captures_iter(string.as_str()){
		variable_index.insert(cap[1].to_string(), cap[2].to_string());
	}
	let string = variable.replace_all(string.as_str(), "").to_string();
	let string = strip_empty_lines(string);
	return (variable_index, string);
}

pub fn find_atrules(string: String) {
	let at_rule = Regex::new(r"@([\w\d\-_+>,#.\[\]: \(\)]*)\s*\t*\n*\{( *\n*\t*)+").unwrap();
	for cap in at_rule.captures_iter(string.as_str()){
		println!("{}", cap[0].to_string());
	}
}

pub fn find_blocks(string: String){
	let block = Regex::new(r"([\w\d\-_+>,#.\[\]:]*)\n* *\t*\{(\n*\t* *[\w\d\-]* *\t*: [\w\d\(\)\-_]*;)+\n* *\t*}").unwrap();
	for cap in block.captures_iter(string.as_str()){
		println!("{}", cap[0].to_string());
	}
}