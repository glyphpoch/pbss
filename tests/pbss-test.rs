use pbss::*;
use std::fs::File;
use std::io::prelude::*;

#[test]
fn test_pbss(){
	let mut file = File::open("tests/result.css").expect("Can't open file");
	let mut result = String::new();
	file.read_to_string(&mut result).expect("Can't read file");
    let arguments: Arguments = Arguments {
        readfile: "tests/test.pbss".to_string(),
        writefile: ":s".to_string()
    };
    let contents = compile(&arguments.readfile, &arguments.writefile);
    assert_eq!(contents, result);
}