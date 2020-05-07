pub mod parser;
use std::env::args;
pub mod file_handling;

pub struct Arguments {
    pub quiet_mode: bool,
	pub readfile: String,
	pub writefile: String,
}

impl Arguments {
    pub fn read() -> Arguments{
    let flags: Vec<String> = args().collect();
    let flags = &flags[1..];
    if flags.len() < 2{
        eprintln!("Error number of arguments");
        std::process::exit(2);
    }
    let read_file = (&flags[flags.len() - 2]).to_string();
    let write_file = (&flags[flags.len() -1]).to_string();
    let mut quiet_mode = false;

    if flags.contains(&"-q".to_string()) || flags.contains(&"--quiet".to_string()){
        quiet_mode = true;
    }
    if quiet_mode == true && write_file == ":s".to_string() {
        eprintln!("Request to redirect to stdout given along with quiet flag");
        std::process::exit(2);
    }

    Arguments {
        quiet_mode: quiet_mode, readfile: read_file, writefile: write_file
    }
}}

pub fn compile(readfile: &String) -> String {
    let contents = parser::read_file(readfile);
    
    let uncomment_string =  parser::strip_comments(contents);
    let raw_string = parser::strip_empty_lines(uncomment_string);
    let (var_index, no_var_str) = parser::track_vars(raw_string);
    let (at_rules, no_at_query_str) = parser::find_atrules(no_var_str);
    let blocks = parser::find_blocks(no_at_query_str);
    let mut out_conts = String::new();

    for block in blocks {
        out_conts.push_str(&parser::resolve_block(block, &var_index))
    }
    for at_rule in at_rules {
        out_conts.push_str(&parser::resolve_block(at_rule, &var_index))
    }
    return out_conts;
}