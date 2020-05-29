use crate::actions::*;
use util::lines::LineType;
use ansi_term::Colour::{Blue, Red};
use crate::State;

pub fn actions(mut state: &mut State) {
    for cl in state.class_line.ltype.clone() {
        match cl {
            LineType::Variable => act_track_vars(&mut state),
            LineType::OneLineComment => act_ol_comment(&mut state),
            LineType::CommentStart => act_ml_comment(&mut state),
            LineType::BlockStart => act_push_contents(&mut state),
            LineType::Style => act_generic(&mut state),
            LineType::BlockEnd => act_push_contents(&mut state),
            LineType::AtRule => act_generic(&mut state),
            LineType::Newline => act_push_contents(&mut state),
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