use crate::parser;
use regex::Regex;

#[derive(Clone, Copy)]
pub enum LineType {
	Variable,
	CommentStart,
	CommentBody,
	CommentEnd,
	BlockStart,
	OneLineComment,
	Style,
	BlockEnd,
	AtRule
}

pub struct Line {
	pub string: Vec<String>,
	pub ltype: Vec<LineType>,
}

pub struct Pattern {
	expression: Regex,
	ptype: LineType
}

impl Pattern {
	fn new(exp: &str, ptype: LineType) -> Pattern{
		Pattern {expression: Regex::new(exp).unwrap(), ptype: ptype }
	}
}

pub fn generate_basic_patterns() -> [Pattern; 6] {
	[
	Pattern::new(r"^\$(\w+[\w\d_\-]*): ([\w \d \(\)\t!,]*);", LineType::Variable),
	Pattern::new(r#"/\*[\w\d~` !@#$%^&()_\-+=|\\\{}\[\]:;""''.,<>]*$"#, LineType::CommentStart),
	Pattern::new(r#"/\*[\w\d~` !@#$%^&()_\-+=|\\\{}\[\]:;""''.,<>]*\*/$"#, LineType::OneLineComment),
	Pattern::new(r#"^[\w\d~` !@#$%^&()_\-+=|\\\{}\[\]:;""''.,<>]*$"#, LineType::CommentBody),
	Pattern::new(r#"^[\w\d~` !@#$%^&()_\-+=|\\\{}\[\]:;""''.,<>]*\*/$"#, LineType::CommentEnd),
	Pattern::new(r"^\t*\s*[\w\d.:-_+> #\(\)\[\]]*\s*\t*\{", LineType::BlockStart),
	]
}

pub fn get_classified_line(line: &str, patterns: &[Pattern]) -> Line{
	let mut ltypes: Vec<LineType> = Vec::new();
	let mut strings: Vec<String> = Vec::new();
	for pattern in patterns {
		for cap in pattern.expression.captures_iter(line){
			strings.push(cap[0].to_string());
			ltypes.push(pattern.ptype);
		}
	}
	return Line {string: strings, ltype: ltypes }
}

pub fn new_compile(readfile: &String, patterns: &[Pattern]) {
    let contents = parser::read_file(readfile);
    let mut count = 0;
    let lcount = contents.lines().count();
    let mut lines = contents.lines();
    while count < lcount {
    	let line = lines.nth(0).unwrap();
    	let class_line = get_classified_line(&line, patterns);
    	for cl in class_line.ltype {
       		match cl {
       			LineType::Variable => println!("{}  Variable", line),
       			LineType::OneLineComment => println!("{}  SignleComment", line), 
       			LineType::CommentStart => {
       				println!("{}", line);
       				loop {
       					count += 1;
       					let nl = contents.lines().nth(count).unwrap();
       					if patterns[3].expression.is_match(nl) {
       						println!("{}  Comment Body", nl);
       					} else {
       						break;
       					}
       				}
       			},
       			LineType::CommentEnd => println!("{}  Comment End", line),
       			LineType::BlockStart => println!("{} Block Start", line),
       			_ => {}
       		}
    	}
    	count += 1;
    }
}

// while count < contents.lines().count() {
//     	let line = lines.nth(count).unwrap();
//     	let class_line = get_classified_line(&line, patterns);

//     	count += 1;
// 	}