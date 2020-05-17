pub mod parser;
pub mod actions;
use regex::Regex;
use std::env::args;
pub mod file_handling;
use filetime::FileTime;
use std::fs::metadata;
pub mod file_include;

// Pbss Version
static PBSS_VERSION: &str = "Pbss-1.3 snap";

// Arguments struct
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

#[derive(Clone, Copy, std::cmp::PartialEq)]
pub enum LineType {
    Variable,
    CommentStart,
    CommentEnd,
    BlockStart,
    OneLineComment,
    Style,
    BlockEnd,
    AtRule,
    OneLineStyle,
    Invalid,
    Newline
}

pub struct Line {
    pub string: String,
    pub ltype: Vec<LineType>,
}

pub struct Pattern {
    expression: Regex,
    ptype: LineType
}

impl Pattern {
    fn new(exp: &str, ptype: LineType) -> Pattern{
        Pattern {expression: Regex::new(exp).unwrap(), ptype: ptype }
    }
}

pub fn generate_basic_patterns() -> [Pattern; 9] {
    [
    Pattern::new(r"^\$(\w+[\w\d_\-]*): *\t*([\w \d \(\)\t!,%]*);", LineType::Variable),
    Pattern::new(r#"/\*[\w\d~` !@#$%^&()_\-+=|\\\{}\[\]:;""''.,<>]*$"#, LineType::CommentStart),
    Pattern::new(r#"^[\w\d~` !@#$%^&()_\-+=|\\\{}\[\]:;""''.,<>]*\*/$"#, LineType::CommentEnd),
    Pattern::new(r#"(/\*[\w\d ~` !@#$%^&\(\)_\-+=|\\\{}\[\]:;""''.,<>]*\*/$)"#, LineType::OneLineComment),
    Pattern::new(r"^\t*\s*[\w\d.:\-_+> #\(\)\[\]]*\s*\t*\{", LineType::BlockStart),
    Pattern::new(r"^\t*\s*[\w\d-]* *\t*: [\w\d\(\)\[\]! $\-]*;",LineType::Style),
    Pattern::new(r"\t*\s*}", LineType::BlockEnd),
    Pattern::new(r"@[\w\d\- \(\):\t$]* *\t*\{", LineType::AtRule),
    Pattern::new(r"^\s*\t*$", LineType::Newline),
    ]
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
    pub var_subs: &'a Regex
}