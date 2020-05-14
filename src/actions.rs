// use ansi_term::Colour::{Red,Blue};
use std::collections::HashMap;
use crate::Pattern;
use crate::{Line,LineType};
use regex::Regex;

pub fn track_vars(line: &String, var_index: &mut HashMap<String, String>, var: &Regex) {
	for cap in var.captures_iter(line) {
		var_index.insert(cap[1].to_string(), cap[2].to_string());
	}
}

pub fn ol_comment_handle(line: &String, comment: &Regex, string: &mut String) {
	println!("R");
	for cap in comment.captures_iter(line) {
		string.push_str(&cap[0]);
		string.push_str("\n");
	}	
}

pub fn actions(class_line: Line, count: &usize, pattern: &[Pattern], mut var_index: &mut HashMap<String, String>, mut contents: &mut String) {

	for cl in class_line.ltype {
        match cl {
            LineType::Variable => track_vars(&class_line.string, &mut var_index, &pattern[0].expression),
            LineType::OneLineComment => ol_comment_handle(&class_line.string, &pattern[2].expression, &mut contents),
            // LineType::CommentStart => {
            //     println!("{}", line);
            //     loop {
            //         let nl = lines[count + 1];
            //         if patterns[3].expression.is_match(nl) {
            //             count += 1;
            //             println!("{}", nl);
            //         } else {
            //             break;
            //         }
            //     }
            // },
            // LineType::CommentEnd => println!("{}", line),
            // LineType::BlockStart => println!("{}", line),
            // LineType::Style => println!("{}", line),
            // LineType::BlockEnd => println!("{}", line),
            // LineType::AtRule => println!("{}", line),
            // LineType::Newline => println!("{}", line),
            // LineType::OneLineStyle => println!("{}", line),
            // LineType::Invalid => {
            //     let line_no = format!("{}", count + 1);
            //     eprintln!("{}| {}", Blue.bold().paint(line_no),
            //         Red.paint(line));
            //     eprintln!("{}", Red.bold().paint("\t|"));
            //     eprintln!("{}", Red.bold().paint("\t:Invalid Line"));
            // }
            _ => {}
        }
    }
}