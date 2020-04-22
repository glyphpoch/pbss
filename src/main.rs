use std::env::args;
use std::path::Path;
use std::process;
mod parser;
use std::collections::HashMap;

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

fn check_readfile(path: &String) -> bool {
	return Path::new(path).exists();
}

fn main() {
	    let arguments: Arguments = Arguments::new();
	    if ! check_readfile(&arguments.readfile) {
	    	println!("Specified path '{}' does not exists in filesystem", &arguments.readfile);
	    	process::exit(1)
	    }

	let css_map: HashMap<String, String> = HashMap::new();
    let contents = parser::read_file(arguments.readfile);
    let blocks = parser::break_blocks(contents);

    for mut b in blocks {
    	parser::strip_newlines(&mut b);
    	let tokens = parser::break_tokens(&mut b);
    }
}
