use crate::util::Pattern;

// pub fn act_comment<'a>(line: &'a String, pattern: &'a Pattern) -> str {
// 	for captures in pattern.expression.captures_iter(line.as_str()){
// 		return captures[0];
// 	}
// }

// pub fn act_variable<'a>(line: &'a String, pattern: &'a Pattern) -> str{
// 	if ! line.starts_with("$"){
// 		for captures in pattern.expression.captures_iter(line.as_str()){
// 			return captures[0];
// 		}
// 	}
// }