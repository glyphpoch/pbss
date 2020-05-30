use util::actions::*;
use util::lines::LineType;
use util::State;

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
            LineType::Invalid => act_invalid(&state),
            _ => {}
        }
    }
}
