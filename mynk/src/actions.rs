use crate::State;
use ansi_term::Colour::{Blue, Red};
use util::lines::LineType;
// use regex::Regex;

fn add_line(cs: &mut State, line: &str) {
    cs.contents.push_str(line);
    cs.contents.push('\n');
}

fn act_resolve_vars(cs: &mut State) {
    for cap in cs.ext_pattern[0].captures_iter(&cs.class_line.string.to_string()) {
        cs.class_line.string = cs
            .class_line
            .string
            .replace(&cap[0], &cs.var_index[&cap[1]]);
    }
}

fn act_calc(cs: &mut State) {
    for cap in cs.ext_pattern[1].captures_iter(&cs.class_line.string.clone()) {
        let num1: isize = cap[1].to_string().parse().expect("Not an integer");
        let num2: isize = cap[3].to_string().parse().expect("Not an integer");
        match &cap[2] {
            "+" => {
                cs.class_line.string = cs
                    .class_line
                    .string
                    .replace(&cap[0], format!("{}", num1 + num2).as_str())
            }
            "-" => {
                cs.class_line.string = cs
                    .class_line
                    .string
                    .replace(&cap[0], format!("{}", num1 - num2).as_str())
            }
            "*" => {
                cs.class_line.string = cs
                    .class_line
                    .string
                    .replace(&cap[0], format!("{}", num1 * num2).as_str())
            }
            "/" => {
                cs.class_line.string = cs
                    .class_line
                    .string
                    .replace(&cap[0], format!("{}", num1 / num2).as_str())
            }
            _ => {}
        }
    }
}

fn act_track_vars(cs: &mut State) {
    for cap in cs.patterns[0]
        .expression
        .captures_iter(&cs.class_line.string)
    {
        cs.var_index.insert(cap[1].to_string(), cap[2].to_string());
    }
}

fn act_ol_comment(mut cs: &mut State) {
    for cap in cs.patterns[3]
        .expression
        .captures_iter(&cs.class_line.string.clone())
    {
        add_line(&mut cs, &cap[0]);
    }
}

fn act_ml_comment(mut cs: &mut State) {
    push_contents(&mut cs);
    loop {
        let nl = cs.lines[*cs.count + &1];
        if !cs.patterns[2].expression.is_match(nl) {
            *cs.count += &1;
            add_line(&mut cs, nl);
        } else {
            add_line(&mut cs, nl);
            break;
        }
    }
}

fn push_contents(mut cs: &mut State) {
    let start = cs.class_line.string.clone();
    add_line(&mut cs, &start);
}

fn act_generic(mut cs: &mut State) {
    act_resolve_vars(&mut cs);
    act_calc(&mut cs);
    push_contents(&mut cs);
}

pub fn actions(mut state: &mut State) {
    for cl in state.class_line.ltype.clone() {
        match cl {
            LineType::Variable => act_track_vars(&mut state),
            LineType::OneLineComment => act_ol_comment(&mut state),
            LineType::CommentStart => act_ml_comment(&mut state),
            LineType::BlockStart => push_contents(&mut state),
            LineType::Style => act_generic(&mut state),
            LineType::BlockEnd => push_contents(&mut state),
            LineType::AtRule => act_generic(&mut state),
            LineType::Newline => push_contents(&mut state),
            LineType::OneLineStyle => act_generic(&mut state),
            LineType::Invalid => {
                let line_no = format!("{}", *state.count + 1);
                eprintln!(
                    "{}| {}",
                    Blue.bold().paint(line_no),
                    Red.paint(&state.class_line.string.to_string())
                );

                eprintln!("{}", Red.bold().paint("\t|"));
                eprintln!("{}", Red.bold().paint("\t:Invalid Line"));
            }
            _ => {}
        }
    }
}
