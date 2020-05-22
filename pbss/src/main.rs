use mynk::generate_basic_patterns;
use mynk::parser::compile;
use mynk::{file_handling, get_file_mod_time, Arguments};
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

#[derive(std::cmp::PartialEq)]
struct Change;

fn compile_file(args: &Arguments, patterns: &[mynk::Pattern]) {
    let contents = compile(&args.readfile, &patterns);
    file_handling::writer(contents, &args);
}

fn main() {
    // The starting point check for arguments and compile, if asked run
    // for watch mode
    let arguments: Arguments = Arguments::read();
    let copy_args = arguments.clone();
    file_handling::check_readfile(&arguments.readfile);
    if arguments.r#override == false {
        file_handling::check_writefile(&arguments.writefile);
    }
    let patterns = generate_basic_patterns();
    compile_file(&arguments, &patterns);
    if arguments.watch == true {
        let (tx, rx) = mpsc::channel();
        let _ = thread::spawn(move || {
            let mut mod_time = get_file_mod_time(&arguments.readfile);

            loop {
                let current_mod = get_file_mod_time(&arguments.readfile);
                if mod_time == current_mod {
                    thread::sleep(Duration::new(1, 0));
                } else {
                    tx.send(Change).expect("Oops. Something went wrong");
                    mod_time = current_mod;
                }
            }
        });
        for rmsg in rx{
            if rmsg == Change {
                compile_file(&copy_args, &patterns);
            }
        }
    }
}
