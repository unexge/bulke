use super::{match_, pattern};
use std::collections::HashMap;

type State<'a> = HashMap<String, &'a str>;

fn eval_expr(expr: &str, state: &State) -> String {
    let mut res = String::new();
    let mut in_variable = false;
    let mut variable = String::new();
    for c in expr.chars() {
        match c {
            '$' => {
                in_variable = true;
                variable.clear();
            }
            'A'..='Z' | 'a'..='z' if in_variable => {
                variable.push(c);
            }
            _ => {
                if in_variable {
                    in_variable = false;
                    if let Some(val) = state.get(&variable) {
                        res.push_str(val);
                    }
                    variable.clear();
                }
                res.push(c);
            }
        }
    }
    res
}

pub fn eval(input: &str, match_: &match_::Match) -> String {
    let state: State = match_
        .0
        .iter()
        .filter_map(|(r, p)| match &r {
            pattern::Rule::Capture(n) => Some((n.clone(), &input[p.0..p.1])),
            _ => None,
        })
        .collect();

    let mut res = input.to_string();

    for (r, p) in &match_.0 {
        if let pattern::Rule::Add(expr) = r {
            let mut expr = expr.to_string();
            expr.push('\n'); // FIXME: how to handle new lines properly?

            let expr = eval_expr(&expr, &state);
            res.insert_str(p.0, &expr)
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluating() {
        let input = "\
qux: baz
foo: bar
bar: 42
qux: foo
";
        let match_ = match_::match_(
            input,
            &pattern::parse(
                "\
foo: bar
bar: {=BAR}
{+answer: $BAR}
",
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(
            eval(input, &match_),
            "\
qux: baz
foo: bar
bar: 42
answer: 42
qux: foo
",
        );
    }
}
