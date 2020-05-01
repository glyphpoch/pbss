use pbss::{Arguments, file_handling, compile};

fn main() {
	let arguments: Arguments = Arguments::read();
	file_handling::check_readfile(&arguments.readfile);
    file_handling::check_writefile(&arguments.writefile);
    compile(&arguments.readfile, &arguments.writefile);
}
