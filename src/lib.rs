pub mod parser;
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

pub fn compile(readfile: &String) -> String {
    // Compile the reading file
    /* Steps involved
      - Read the file
      - Strip out comments
      - Remove empty lines
      - Check for includes
      - Track and put variables in a HashMap and remove the lines from the reading file
      - Find all at rules @. and put them for later use in a vector
      - Read normal style blocks
      - Read and resolve all variables in the blocks and add to a master contents, out_conts
    */
    let mut contents = parser::read_file(readfile);
    parser::strip_comments(&mut contents);
    parser::strip_empty_lines(&mut contents);
    file_include::check_includes(&mut contents);
    let var_index = parser::track_vars(&mut contents);
    let at_rules = parser::find_atrules(&mut contents);
    let blocks = parser::find_blocks(contents);
    let mut out_conts = String::new();

    for block in blocks {
        out_conts.push_str(&parser::resolve_block(block, &var_index))
    }
    for at_rule in &at_rules {
        out_conts.push_str(&parser::resolve_block(at_rule.to_string(), &var_index))
    }
    return out_conts;
}

pub fn get_file_mod_time(file: &String) -> i64 {
    // Get the last modification  time of a file for watch mode
    let file_meta = metadata(file).expect("Can't get file metadata");
    let last_mod_time = FileTime::from_last_modification_time(&file_meta).seconds();
    last_mod_time
}
