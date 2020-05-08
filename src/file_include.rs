use regex::Regex;
use crate::compile;
use crate::file_handling::check_readfile;
use crate::parser::read_file;

pub fn check_includes(contents: &mut String) -> String {
	let include = Regex::new(r"include\((.+)\);").unwrap();
	for cap in include.captures_iter(&contents.to_string()){
		*contents = include_files(&cap[1], contents);
	}
	contents.to_string()
}

pub fn include_files(file: &str, fc: &String) -> String{
	check_readfile(&file.to_string());
	let path = &format!("include({});", file);
	let contents = read_file(&file.to_string());
	let new_data = fc.replace(path, &contents);
	new_data.to_string()
}