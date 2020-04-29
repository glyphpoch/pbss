use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
// use crate::util::{Pattern,Line,LineType};
// use crate::actions::*;

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

pub fn strip_empty_lines(string: String) {
	let newline = Regex::new(r"^\n+$").unwrap();
	let mut raw_string = String::new();
	for mut line in string.lines(){
		let edit_line = &newline.replace_all(line, "");
		if ! edit_line.is_empty() {
			raw_string.push_str(edit_line);
			raw_string.push_str("\n");
		}
	}
	println!("{}", raw_string);
}

// pub fn classify(contents: &str, patterns: &Vec<Pattern>) -> Option<Line>{
// 	let mut passed: Vec<LineType> = Vec::new();
// 	for pattern in patterns{
// 		if pattern.expression.is_match(contents) {
// 			passed.push(pattern.ptype)
// 		}
// 	}
// 	if passed.len() > 0{
// 		return Some(Line { line: contents.to_string(), ltype: passed })
// 	}
// 	return None
// }

// pub fn line_action(line: Line, patterns: &Vec<Pattern>){
// 	let mut gen_line = String::new();
// 	for pass in line.ltype {
// 		match pass {
// 			LineType::Style => gen_line.push_str(act_variable(&line.line, &patterns[2])),
// 			LineType::Comment => gen_line.push_str(act_comment(&line.line, &patterns[5])),
// 			_ => {},
// 		}
// 	}
// }