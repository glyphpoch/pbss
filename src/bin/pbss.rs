use pbss::{compile, file_handling, get_file_mod_time, Arguments};
use std::thread::sleep;
use std::time::Duration;

fn start_watch(args: &Arguments) {
    let mut mod_time = get_file_mod_time(&args.readfile);

    loop {
        let current_mod = get_file_mod_time(&args.readfile);
        if mod_time == current_mod {
            sleep(Duration::new(1, 0));
        } else {
            let contents = compile(&args.readfile);
            file_handling::writer(contents, &args);
            mod_time = current_mod;
        }
    }
}

fn main() {
    let arguments: Arguments = Arguments::read();
    file_handling::check_readfile(&arguments.readfile);
    if arguments.r#override == false {
        file_handling::check_writefile(&arguments.writefile);
    }

    let contents = compile(&arguments.readfile);
    file_handling::writer(contents, &arguments);
    if arguments.watch == true {
        start_watch(&arguments)
    }
}
