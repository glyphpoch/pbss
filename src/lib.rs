pub mod parser;
use std::env::args;
pub mod file_handling;

pub struct Arguments {
	pub readfile: String,
	pub writefile: String,
}

impl Arguments {
	pub fn read() -> Arguments {
		let flags: Vec<String> = args().collect();
		let flags = &flags[1..];
		let arguments = Arguments {
			readfile: flags[0].to_string(),
			writefile: flags[1].to_string()
	};
	arguments
}}

pub fn compile(readfile: &String, writefile: &String) -> String {
    let contents = parser::read_file(readfile);
    let uncomment_string =  parser::strip_comments(contents);
    let raw_string = parser::strip_empty_lines(uncomment_string);
    let (var_index, no_var_str) = parser::track_vars(raw_string);
    let (at_rules, no_at_query_str) = parser::find_atrules(no_var_str);
    let blocks = parser::find_blocks(no_at_query_str);
    let mut out_conts = String::new();
    for block in blocks {
        out_conts.push_str(parser::resolve_block(block, &var_index)
            .as_str())
    }
    return out_conts;
}