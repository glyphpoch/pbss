pub mod actions;
pub mod lines;
pub mod words;
use regex::Regex;

pub struct State<'a> {
    pub class_line: lines::Line,
    pub count: &'a mut usize,
    pub patterns: &'a [lines::Pattern],
    pub var_index: &'a mut std::collections::HashMap<String, String>,
    pub contents: &'a mut String,
    pub lines: &'a Vec<&'a str>,
    pub ext_pattern: &'a [Regex; 2],
}
