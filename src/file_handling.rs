use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::path::Path;
use std::process;
use std::io::{stdin,stdout};

pub fn check_readfile(path: &String) {
	if ! Path::new(path).exists() {
		println!("Specified path '{}' does not exists in filesystem", path);
	    process::exit(1)
	}
}

pub fn writer(map: HashMap<String, HashMap<String, String>>, sequence: Vec<String>) -> String{

	let mut block = String::new();

	for item in sequence.iter(){
		block.push_str(format!("{} {{\n", item).as_str());
		for (prop, val) in map[item].keys().zip(map[item].values()){
			block.push_str(format!("    {}: {};\n", prop, val).as_str());
		}
		block.push_str("}\n");
	}

	return block;
}

pub fn write_blocks(blocks: String, output: String){
	if output == ":s" {
		println!("{}", blocks);
	} else {
		check_writefile(&output);
		let mut file = File::create(output).expect("Can't create file");
		file.write_all(blocks.as_bytes()).expect("Can't write file");
	}
}

pub fn check_writefile(path: &String){
	if Path::new(path).exists(){
		print!("It seems {} exists in file system. Overrite the file [Y/n] ", path);
		stdout().flush();
		let mut ans = String::new();
		stdin().read_line(&mut ans).expect("Unable to read input");

		if ! (ans == "" || ans == "Y" || ans == "y"){
			process::exit(2);
		}
	}
}