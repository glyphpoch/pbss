use crate::State;
use crate::LineType;
use regex::Regex;
use ansi_term::Colour::{Red,Blue};

pub fn act_track_vars(cs: &mut State) {
    for cap in cs.patterns[0].expression.captures_iter(&cs.class_line.string){
		cs.var_index.insert(cap[1].to_string(), cap[2].to_string());
	}
}

pub fn act_ol_comment(cs: &mut State) {
	for cap in cs.patterns[3].expression.captures_iter(&cs.class_line.string) {
        cs.contents.push_str(&cap[0]);
        cs.contents.push_str("\n")
    }
}

pub fn act_ml_comment(cs: &mut State) {
    cs.contents.push_str(&cs.class_line.string);
    cs.contents.push_str("\n");
    loop {
        let nl = cs.lines[*cs.count + &1];
        if ! cs.patterns[2].expression.is_match(nl) {
            *cs.count += &1;
            cs.contents.push_str(nl);
            cs.contents.push_str("\n");
        } else {
            cs.contents.push_str(nl);
            cs.contents.push_str("\n");
            break;
        }
    }
}

pub fn act_block_start(cs: &mut State) {
    cs.contents.push_str(&cs.class_line.string);
    cs.contents.push_str("\n");
}

pub fn act_block_start(state: &State, ve: &Regex) {
    for ve.captures_iter(state.class_line.string){

    }
}

pub fn actions(mut state: &mut State, var_exp: &Regex) {
	for cl in state.class_line.ltype.clone() {
        match cl {
            LineType::Variable => act_track_vars(&mut state),
            LineType::OneLineComment => act_ol_comment(&mut state),
            LineType::CommentStart => act_ml_comment(&mut state),
            LineType::BlockStart => act_block_start(&mut state),
            // LineType::Style => println!("{}", line),
            // LineType::BlockEnd => println!("{}", line),
            // LineType::AtRule => println!("{}", line),
            // LineType::Newline => println!("{}", line),
            // LineType::OneLineStyle => println!("{}", line),
            LineType::Invalid => {
                let line_no = format!("{}", *state.count + 1);
                eprintln!("{}| {}", Blue.bold().paint(line_no)
                    ,Red.paint(&state.class_line.string.to_string()));

                eprintln!("{}", Red.bold().paint("\t|"));
                eprintln!("{}", Red.bold().paint("\t:Invalid Line"));
            }
            _ => {}
        }
    }
}