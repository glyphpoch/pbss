use std::env::args;
mod parser;
mod file_handling;
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

fn main() {
	let arguments: Arguments = Arguments::new();
	file_handling::check_readfile(&arguments.readfile);
	// file_handling

	let mut style_map: HashMap<String, HashMap<String, String> > = HashMap::new();
    let contents = parser::read_file(arguments.readfile);
    let mut selector_sequence: Vec<String> = Vec::new();
    let blocks = parser::break_blocks(contents);

    for mut b in blocks {
    	parser::strip_newlines(&mut b);
    	let tokens = parser::break_tokens(&mut b);
    	let mut prop_map: HashMap<String, String> = HashMap::new();
    	let (keys, values) = parser::format_property(tokens.1.to_string());
    	selector_sequence.push(tokens.0.to_string());
    	for (key, val) in keys.iter().zip(values.iter()){
    		prop_map.insert(key.to_string(), val.to_string());
    	}
    	style_map.insert(tokens.0.to_string(), prop_map);
    }

    let blocks = file_handling::writer(style_map, selector_sequence);
    file_handling::write_blocks(blocks, arguments.writefile)
}
