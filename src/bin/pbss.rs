use std::env::args;
use std::collections::HashMap;
use pbss::{parser,file_handling,util};

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
    parser::strip_empty_lines(uncomment_string)

    // println!("{}", contents);
    // let basic_patterns = util::Pattern::base();
    // for line in contents.lines(){
    //     let classed_line = parser::classify(line, &basic_patterns);
    //     match classed_line {
    //         Some(l) => parser::line_action(l, &basic_patterns),
    //         None => {}
    //     }
}
