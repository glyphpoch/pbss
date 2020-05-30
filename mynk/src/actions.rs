use crate::State;
// use regex::Regex;

fn add_line(cs: &mut State, line: &str) {
    cs.contents.push_str(line);
    cs.contents.push('\n');
}

pub fn act_resolve_vars(cs: &mut State) {
    for cap in cs.ext_pattern[0].captures_iter(&cs.class_line.string.to_string()) {
        cs.class_line.string = cs
            .class_line
            .string
            .replace(&cap[0], &cs.var_index[&cap[1]]);
    }
}

pub fn act_calc(cs: &mut State) {
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
                if num1 % num2 == 0 {
                    cs.class_line.string = cs
                    .class_line
                    .string
                    .replace(&cap[0], format!("{}", num1 / num2).as_str())
                    } else {
                    let num1 = num1 as f64;
                    let num2 = num2 as f64;
                    cs.class_line.string = cs
                    .class_line
                    .string
                    .replace(&cap[0], format!("{}", num1 / num2).as_str())
                }},
            _ => {}
        }
    }
}

pub fn act_track_vars(cs: &mut State) {
    for cap in cs.patterns[0]
        .expression
        .captures_iter(&cs.class_line.string)
    {
        cs.var_index.insert(cap[1].to_string(), cap[2].to_string());
    }
}

pub fn act_ol_comment(mut cs: &mut State) {
    for cap in cs.patterns[3]
        .expression
        .captures_iter(&cs.class_line.string.clone())
    {
        add_line(&mut cs, &cap[0]);
    }
}

pub fn act_ml_comment(mut cs: &mut State) {
    act_push_contents(&mut cs);
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

pub fn act_push_contents(mut cs: &mut State) {
    let start = cs.class_line.string.clone();
    add_line(&mut cs, &start);
}

pub fn act_generic(mut cs: &mut State) {
    act_resolve_vars(&mut cs);
    act_calc(&mut cs);
    act_push_contents(&mut cs);
}