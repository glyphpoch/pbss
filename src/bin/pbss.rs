use pbss::{Arguments, file_handling, compile};

fn main() {
	let arguments: Arguments = Arguments::read();
	file_handling::check_readfile(&arguments.readfile);
    file_handling::check_writefile(&arguments.writefile);
    let contents = compile(&arguments.readfile);
    file_handling::writer(contents, &arguments)
}
