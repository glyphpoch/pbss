use pbss::*;
use std::fs::File;

#[test]
fn test_pbss(){
    let arguments: Arguments = Arguments {
        readfile: "test.pbss".to_string(),
        writefile: ":s".to_string()
    };
    compile(&arguments.readfile, &arguments.writefile);
}
