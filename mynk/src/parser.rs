use crate::{actions, State};
use crate::file_include;
use util::lines::{Line, LineType, Pattern};
use std::collections::HashMap;

use std::fs::File;
use std::io::prelude::*;

pub fn read_file(file: &String) -> String {
    let mut file = File::open(file).expect("Can't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Can't read file");

    return contents;
}

pub fn get_classified_line(line: &str, patterns: &[Pattern]) -> Line {
    let mut ltypes: Vec<LineType> = Vec::new();
    for pattern in patterns {
        if pattern.expression.is_match(line) {
            ltypes.push(pattern.ptype);
        }
    }
    if ltypes.contains(&LineType::BlockStart) && ltypes.contains(&LineType::BlockEnd) {
        ltypes = vec![LineType::OneLineStyle];
    }
    if ltypes.len() == 0 {
        ltypes.push(LineType::Invalid);
    }
    Line {
        string: line.to_string(),
        ltype: ltypes,
    }
}

pub fn compile(file: &String, patterns: &[Pattern]) -> String {
    let mut contents = read_file(file);
    file_include::check_includes(&mut contents);
    let mut count = 0;
    let lcount = contents.lines().count();
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut var_index: HashMap<String, String> = HashMap::new();
    let mut contents: String = String::new();
    let ext_pattern = util::words::generate_word_patterns();
    
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
            ext_pattern: &ext_pattern,
        };
        actions::actions(&mut state);
        count += 1;
    }
    contents
}
