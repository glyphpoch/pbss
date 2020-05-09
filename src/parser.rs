use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn read_file(file: &String) -> String {
    let mut file = File::open(file).expect("Can't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Can't read file");

    return contents;
}

pub fn strip_comments(contents: &mut String) -> String {
    let comment = Regex::new(r#"/\*[\d\w \n\t!@#$%^&*\(\)_\-\+=~`|\\\[\]'":;<>,./?]*\*/"#).unwrap();
    let text = comment.replace_all(&contents, "");
    return text.to_string();
}

pub fn strip_empty_lines(string: &mut String) -> String {
    let newline = Regex::new(r"^ *\n*$").unwrap();
    let mut raw_string = String::new();
    for line in string.lines() {
        let edit_line = &newline.replace_all(line, "");
        if !edit_line.is_empty() {
            raw_string.push_str(edit_line);
            raw_string.push_str("\n");
        }
    }
    return raw_string;
}

pub fn track_vars(mut string: &mut String) -> HashMap<String, String> {
    let variable = Regex::new(r"\$(\w+[\w\d_\-]*) *\t*: *\t*([\d\w_\-\(\),]*);").unwrap();
    let mut variable_index: HashMap<String, String> = HashMap::new();
    for cap in variable.captures_iter(string.as_str()) {
        variable_index.insert(cap[1].to_string(), cap[2].to_string());
    }
    *string = variable.replace_all(string.as_str(), "").to_string();
    *string = strip_empty_lines(&mut string);
    variable_index
}

pub fn find_atrules(mut string: &mut String) -> Vec<String> {
    let at_rule = Regex::new(r"@([\w\d\-,: \(\)]*)\s*\t*\n*\{\n*\t* *[\w\d\-_+>,#.\[\]:]*\n* *\t*\{(\n*\t* *[\w\d\-]* *\t*: [\w\d\(\)\-_$]*;)+\n* *\t*}\n*\s*\t*}").unwrap();
    let mut queries: Vec<String> = Vec::new();

    for cap in at_rule.captures_iter(string.as_str()) {
        queries.push(cap[0].to_string());
    }
    *string = at_rule.replace(string.as_str(), "").to_string();
    *string = strip_empty_lines(&mut string);
    return queries;
}

pub fn find_blocks(string: String) -> Vec<String> {
    let block = Regex::new(
        r"([\w\d\-_+>,#.\[\]:]*)\n* *\t*\{(\n*\t* *[\w\d\-]* *\t*: [\w\d\(\)\-_$]*;)+\n* *\t*}",
    )
    .unwrap();
    let mut blocks: Vec<String> = Vec::new();
    for cap in block.captures_iter(string.as_str()) {
        blocks.push(cap[0].to_string());
    }
    return blocks;
}

pub fn resolve_block(block: String, var_index: &HashMap<String, String>) -> String {
    let var = Regex::new(r"\$(\w+[\w\d_\-]*)").unwrap();
    let mut compiled_block = String::new();
    for line in block.lines() {
        if var.is_match(line) {
            for cap in var.captures_iter(line) {
                let line = &line.replace(&cap[0], &var_index[&cap[1]]);
                compiled_block.push_str(line.as_str());
                compiled_block.push_str("\n");
            }
        } else {
            compiled_block.push_str(line);
            compiled_block.push_str("\n");
        }
    }
    return compiled_block;
}
