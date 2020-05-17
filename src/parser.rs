use crate::{Pattern,LineType,Line,State};
use std::collections::HashMap;
use crate::actions;

use std::fs::File;
use std::io::prelude::*;

pub fn read_file(file: &String) -> String {
    let mut file = File::open(file).expect("Can't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Can't read file");

    return contents;
}

pub fn get_classified_line(line: &str, patterns: &[Pattern]) -> Line{
    let mut ltypes: Vec<LineType> = Vec::new();
    for pattern in patterns {
        if pattern.expression.is_match(line){
            ltypes.push(pattern.ptype);
        }
    }
    if ltypes.contains(&LineType::BlockStart) && ltypes.contains(&LineType::BlockEnd) {
        ltypes = vec![LineType::OneLineStyle];
    }
    if ltypes.len() == 0 {
        ltypes.push(LineType::Invalid);
    }
    Line{ string: line.to_string(), ltype: ltypes }
}

pub fn compile(readfile: &String, patterns: &[Pattern]) -> String{
    let contents = read_file(readfile);
    let mut count = 0;
    let lcount = contents.lines().count();
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut var_index: HashMap<String, String> = HashMap::new();
    let mut contents: String = String::new();
    let var_subs_exp = regex::Regex::new(r"\$(\w+[\w\d_\-]*)*").unwrap();
    while count < lcount {
        let line = lines[count];
        let class_line = get_classified_line(&line, patterns);
        let mut state = State {
            class_line: class_line,
            count: &mut count,
            patterns: patterns,
            var_index: &mut var_index,
            contents: &mut contents,
            lines: &lines,
            var_subs: &var_subs_exp
        };
        actions::actions(&mut state, &var_subs_exp);
        count += 1;
    }
    contents
}