use std::io::prelude::*;
use std::path::Path;
use std::process;
use std::io::{stdin,stdout};
use std::fs::File;

pub fn check_readfile(path: &String) {
	if ! Path::new(path).exists() {
		println!("Specified path '{}' does not exists in filesystem", path);
	    process::exit(1)
	}
}

pub fn writer(blocks: String, output: String){
	if output == ":s" {
		println!("{}", blocks);
	} else {
		let mut file = File::create(output).expect("Can't create file");
		file.write_all(blocks.as_bytes()).expect("Can't write file");
	}
}

pub fn check_writefile(path: &String){
	if Path::new(path).exists(){
		print!("It seems '{}' exists in file system. Override the file [Y/n] ", path);
		stdout().flush();
		let mut ans = String::new();
		stdin().read_line(&mut ans).expect("Unable to read input");
		let ans = ans.trim();

		if ! (ans == "".to_string() || ans == "Y".to_string() || ans == "y".to_string()){
			println!("Ovrride cancelled");
			println!("Got result {}", ans);
			process::exit(2);
		}
	}
}