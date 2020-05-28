use regex::Regex;

pub struct Pattern {
    pub expression: Regex,
    pub ptype: LineType,
}

impl Pattern {
    fn new(exp: &str, ptype: LineType) -> Pattern {
        Pattern {
            expression: Regex::new(exp).unwrap(),
            ptype: ptype,
        }
    }
}

#[derive(Clone, Copy, std::cmp::PartialEq)]
pub enum LineType {
    Variable,
    CommentStart,
    CommentEnd,
    BlockStart,
    OneLineComment,
    Style,
    BlockEnd,
    AtRule,
    OneLineStyle,
    Invalid,
    Newline,
}

pub struct Line {
    pub string: String,
    pub ltype: Vec<LineType>,
}

pub fn generate_basic_patterns() -> [Pattern; 9] {
    [
        Pattern::new(
            r"^\$(\w+[\w\d_\-]*): *\t*([\w \d \(\)\t!,%]*);",
            LineType::Variable,
        ),
        Pattern::new(
            r#"/\*[\w\d~` !@#$%^&()_\-+=|\\\{}\[\]:;""''.,<>]*$"#,
            LineType::CommentStart,
        ),
        Pattern::new(
            r#"^[\w\d~` !@#$%^&()_\-+=|\\\{}\[\]:;""''.,<>]*\*/$"#,
            LineType::CommentEnd,
        ),
        Pattern::new(
            r#"^(/\*[\w\d ~` !@#$%^&\(\)_\-+=|\\\{}\[\]:;""''.,<>]*\*/$)"#,
            LineType::OneLineComment,
        ),
        Pattern::new(
            r"^\t*\s*[\w\d.:\-_+> #\(\)\[\]*]*\s*\t*\{",
            LineType::BlockStart,
        ),
        Pattern::new(
            r"^\t*\s*[\w\d\-]* *\t*: [\w\d\(\)\[\]! $\-,+/*]*;",
            LineType::Style,
        ),
        Pattern::new(r"\t*\s*}", LineType::BlockEnd),
        Pattern::new(r"@[\w\d\- \(\):\t$]* *\t*\{", LineType::AtRule),
        Pattern::new(r"^\s*\t*$", LineType::Newline),
    ]
}
