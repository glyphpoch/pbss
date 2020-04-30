use std::env::args;
use std::collections::HashMap;
use pbss::{parser,file_handling};

struct Arguments {
	readfile: String,
	writefile: String,
}

impl Arguments {
	fn new() -> Arguments {
		let flags: Vec<String> = args().collect();
		let flags = &flags[1..];
		let arguments = Arguments {
			readfile: flags[0].to_string(),
			writefile: flags[1].to_string()
	};
	arguments
}}

fn main() {
	let arguments: Arguments = Arguments::new();
    let mut style_map: HashMap<String, HashMap<String, String> > = HashMap::new();

	file_handling::check_readfile(&arguments.readfile);
    let contents = parser::read_file(&arguments.readfile);
    let uncomment_string =  parser::strip_comments(contents);
    let raw_string = parser::strip_empty_lines(uncomment_string);
    let (var_index, no_var_str) = parser::track_variables(raw_string);
    parser::find_atrules(no_var_str);
    // parser::find_blocks(no_var_str);
}
