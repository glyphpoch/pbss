use std::env::args;
use std::collections::HashMap;
use pbss::{parser,file_handling};

struct Arguments {
	readfile: String,
	writefile: String,
}

impl Arguments {
	fn read() -> Arguments {
		let flags: Vec<String> = args().collect();
		let flags = &flags[1..];
		let arguments = Arguments {
			readfile: flags[0].to_string(),
			writefile: flags[1].to_string()
	};
	arguments
}}

fn compile(readfile: &String) {
    let contents = parser::read_file(readfile);
    let uncomment_string =  parser::strip_comments(contents);
    let raw_string = parser::strip_empty_lines(uncomment_string);
    let (var_index, no_var_str) = parser::track_vars(raw_string);
    let (at_rules, no_at_query_str) = parser::find_atrules(no_var_str);
    let blocks = parser::find_blocks(no_at_query_str);
    parser::resolve_blocks(blocks[0].as_str(), &var_index);
}

fn main() {
	let arguments: Arguments = Arguments::read();
	file_handling::check_readfile(&arguments.readfile);
    compile(&arguments.readfile);
}
