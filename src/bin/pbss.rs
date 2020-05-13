use pbss::{file_handling, get_file_mod_time, Arguments};
use pbss::new_parser::{new_compile, generate_basic_patterns};
use std::thread::sleep;
use std::time::Duration;

fn start_watch(args: &Arguments) {
    // Start watch mode run a loop untill the program is asked to quit
    let mut mod_time = get_file_mod_time(&args.readfile);

    loop {
        let current_mod = get_file_mod_time(&args.readfile);
        if mod_time == current_mod {
            sleep(Duration::new(1, 0));
        } else {
            // let contents = compile(&args.readfile);
            // file_handling::writer(contents, &args);
            mod_time = current_mod;
        }
    }
}

fn main() {
    // The starting point check for arguments and compile, if asked run
    // for watch mode
    let arguments: Arguments = Arguments::read();
    file_handling::check_readfile(&arguments.readfile);
    if arguments.r#override == false {
        file_handling::check_writefile(&arguments.writefile);
    }
    let patterns = generate_basic_patterns();
    let _contents = new_compile(&arguments.readfile, &patterns);
    // file_handling::writer(contents, &arguments);
    if arguments.watch == true {
        start_watch(&arguments)
    }
}
