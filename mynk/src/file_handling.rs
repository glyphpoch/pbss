use crate::Arguments;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, stdout};
use std::path::Path;
use std::process;

pub fn check_readfile(path: &String) {
    // Check if the readfile exists if not immediately quit with error
    if !Path::new(path).exists() {
        println!("Specified path '{}' does not exists in filesystem", path);
        process::exit(1)
    }
}

pub fn writer(contents: String, arguments: &Arguments) {
    // Write all contents to stdout if :s is given as writefile or a file to write
    let output = &arguments.writefile;
    if output == ":s" {
        println!("{}", contents);
    } else {
        let mut file = File::create(&output).expect("Can't create file");
        file.write_all(contents.as_bytes())
            .expect("Can't write file");
        if !arguments.quiet == true {
            println!("Compiled {} and wrote to {}", arguments.readfile, output);
        }
    }
}

pub fn check_writefile(path: &String) {
    // Check if the there is a filename similar to the writefile, ask to check if it should be overriden
    if Path::new(path).exists() {
        print!(
            "It seems '{}' exists in file system. Override the file [Y/n] ",
            path
        );
        let _ = stdout().flush();
        let mut ans = String::new();
        stdin().read_line(&mut ans).expect("Unable to read input");
        let ans = ans.trim();

        if !(ans == "".to_string() || ans == "Y".to_string() || ans == "y".to_string()) {
            println!("Ovrride cancelled");
            println!("Got result {}", ans);
            process::exit(2);
        }
    }
}
