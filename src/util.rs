use regex::Regex;

#[derive(Copy, Clone)]
pub enum LineType {
	Variable,
	Comment,
	BlockOpen,
	BlockClose,
	Style,
	AtRule,
}

impl LineType {
	pub fn in_string(&self) -> &str {
		match self {
			LineType::Variable => "Variable",
			LineType::Comment => "Comment",
			LineType::BlockClose => "BlockClose",
			LineType::BlockOpen => "BlockOpen",
			LineType::Style => "Style",
			LineType::AtRule => "AtRule",
		}
	}
}

pub struct Pattern {
	pub expression: Regex,
	pub ptype: LineType
}

pub struct Line{
	pub line: String,
	pub ltype: Vec<LineType>
}

impl Pattern {
	pub fn new(exp: &str, ptype: LineType) -> Pattern {
		return Pattern{expression: Regex::new(exp).unwrap(), ptype: ptype}
	}
	pub fn base() -> Vec<Pattern>{
		return vec![
		Pattern::new(r"\$(\w+[\w\d_-]*)*\t*: *\t*([\w\d\(\) \t!,(//)]*);", LineType::Variable),
		Pattern::new(r"([\w\- \.:\d]*) *\t*\{", LineType::BlockOpen),
		Pattern::new(r" *\t*(\w+[\w\d \-]*) *\t*: ([\w\d\(\) \t!,(//)\$]*);", LineType::Style),
		Pattern::new(r"\}", LineType::BlockClose),
		Pattern::new(r"@\w+([\w\-\d \(\):]*) *\t*\{", LineType::AtRule),
		Pattern::new(r"//(.*)", LineType::Comment),
		]
	}
}