pub mod actions;
pub mod parser;
use regex::Regex;
use std::env::args;
pub mod file_handling;
use filetime::FileTime;
use std::fs::metadata;
pub mod file_include;
use util::lines::{Line, Pattern};

// Pbss Version
static PBSS_VERSION: &str = "Pbss-2.0 Beryllium";

// Arguments struct
#[derive(Clone)]
pub struct Arguments {
    pub quiet: bool,
    pub watch: bool,
    pub r#override: bool,
    pub readfile: String,
    pub writefile: String,
}

impl Arguments {
    // Read arguments and enable and disable various features
    pub fn read() -> Arguments {
        let flags: Vec<String> = args().collect();
        let flags = &flags[1..];

        if flags.contains(&"-v".to_string()) || flags.contains(&"--version".to_string()) {
            println!("{}", PBSS_VERSION);
            std::process::exit(0);
        }

        if flags.len() < 2 {
            eprintln!("Error number of arguments");
            std::process::exit(2);
        }
        let read_file = (&flags[flags.len() - 2]).to_string();
        let write_file = (&flags[flags.len() - 1]).to_string();

        let mut quiet_mode = false;
        let mut watch_mode = false;
        let mut r#override = false;

        for para in &flags[..flags.len() - 2] {
            match para.as_str() {
                "-w" => watch_mode = true,
                "-q" => quiet_mode = true,
                "--override" => r#override = true,
                _ => println!("Invalid Argument {}", para),
            }
        }

        if quiet_mode == true && write_file == ":s".to_string() {
            eprintln!("Request to redirect to stdout given along with quiet flag");
            std::process::exit(2);
        }

        Arguments {
            quiet: quiet_mode,
            readfile: read_file,
            writefile: write_file,
            watch: watch_mode,
            r#override: r#override,
        }
    }
}

pub fn get_file_mod_time(file: &String) -> i64 {
    // Get the last modification  time of a file for watch mode
    let file_meta = metadata(file).expect("Can't get file metadata");
    let last_mod_time = FileTime::from_last_modification_time(&file_meta).seconds();
    last_mod_time
}

pub struct State<'a> {
    pub class_line: Line,
    pub count: &'a mut usize,
    pub patterns: &'a [Pattern],
    pub var_index: &'a mut std::collections::HashMap<String, String>,
    pub contents: &'a mut String,
    pub lines: &'a Vec<&'a str>,
    pub ext_pattern: &'a [Regex; 2],
}
