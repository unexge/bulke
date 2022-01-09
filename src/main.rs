use std::collections::HashMap;

fn main() {}

#[derive(Debug, Clone)]
enum Atom {
    Match(String),
    Capture(String),
    Add(String),
}

type Rule = Vec<Atom>;
type Rules = Vec<Rule>;

fn parse_rule(input: &str) -> Rule {
    struct State {
        stack: Vec<char>,
        bracket: bool,
        equal: bool,
        plus: bool,
    }

    let mut state = State {
        stack: vec![],
        bracket: false,
        equal: false,
        plus: false,
    };

    let make_atom = |state: &mut State| -> Atom {
        let atom = match (state.bracket, state.equal, state.plus) {
            (true, true, _) => Atom::Capture(state.stack.clone().into_iter().collect()),
            (true, _, true) => Atom::Add(state.stack.clone().into_iter().collect()),
            _ => Atom::Match(state.stack.clone().into_iter().collect()),
        };

        state.stack.clear();
        state.bracket = false;
        state.equal = false;
        state.plus = false;

        atom
    };
    let mut atoms = vec![];

    for c in input.to_string().chars() {
        match c {
            '{' => {
                if !state.stack.is_empty() {
                    atoms.push(make_atom(&mut state));
                }
                state.bracket = true;
            }
            '=' if state.bracket => {
                state.equal = true;
            }
            '+' if state.bracket => {
                state.plus = true;
            }
            '}' if state.bracket => {
                atoms.push(make_atom(&mut state));
            }
            c => {
                state.stack.push(c);
            }
        }
    }

    if !state.stack.is_empty() {
        atoms.push(make_atom(&mut state));
    }

    atoms
}

fn parse_rules(input: &str) -> Rules {
    input.lines().map(parse_rule).collect()
}

fn eval(line: &str, state: &HashMap<String, String>) -> String {
    let mut res = String::new();

    let mut in_variable = false;
    let mut variable = String::new();
    for c in line.chars() {
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

pub fn replace(files: &[&str], input: &str) -> Vec<String> {
    let rules = dbg!(parse_rules(input));
    let mut res = vec![];

    for file in files {
        let mut state = HashMap::new();
        let mut matching = false;
        let mut total_matches = 0usize;
        let total_match_rules: usize = rules
            .iter()
            .map(|r| r.iter().filter(|a| matches!(a, Atom::Match(_))).count())
            .sum();
        let rules = rules.clone();
        let mut current_rule = 0usize;
        let mut res_file: String = file
            .lines()
            .map(|line| {
                let mut line = line.to_string();
                let mut remaining_line = line.clone();
                if current_rule >= rules.len() {
                    return line;
                }

                for atom in &rules[current_rule] {
                    match atom {
                        Atom::Match(pattern) => {
                            if let Some(remaining) = line.strip_prefix(pattern) {
                                matching = true;
                                current_rule += 1;
                                total_matches += 1;
                                remaining_line = remaining.to_string();
                            } else {
                                matching = false;
                                current_rule = 0;
                                total_matches = 0;
                            }
                        }
                        Atom::Capture(name) => {
                            if matching {
                                state.insert(name.clone(), remaining_line.clone());
                            }
                        }
                        Atom::Add(new) => {
                            if matching {
                                line.push('\n');
                                line.push_str(&eval(new, &state));
                            }
                        }
                    }
                }
                line
            })
            .collect::<Vec<_>>()
            .join("\n");
        if current_rule < rules.len() {
            for rule in &rules[current_rule..] {
                for atom in rule {
                    match atom {
                        Atom::Match(_) => {}
                        Atom::Capture(_) => {}
                        Atom::Add(new) => {
                            if total_match_rules == total_matches && matching {
                                res_file.push('\n');
                                res_file.push_str(&eval(new, &state));
                            }
                        }
                    }
                }
            }
        }
        res.push(res_file);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_a_new_line() {
        let files = vec![
            r#"
region: ap
country: jp
"#
            .trim(),
            r#"
region: eu
country: de
            "#
            .trim(),
            r#"
region: mena
country: tr
            "#
            .trim(),
            r#"
region: ap
            "#
            .trim(),
        ];

        let input = r#"
country: {=COUNTRY}
{+enabled: true}
"#
        .trim();

        let result = replace(&files, input);

        assert_eq!(
            result,
            vec![
                r#"
region: ap
country: jp
enabled: true
            "#
                .trim(),
                r#"
region: eu
country: de
enabled: true
            "#
                .trim(),
                r#"
region: mena
country: tr
enabled: true
            "#
                .trim(),
                r#"
region: ap
            "#
                .trim(),
            ]
        );
    }

    #[test]
    fn using_captured_variable() {
        let files = vec![
            r#"
region: ap
country: jp
"#
            .trim(),
            r#"
region: eu
country: de
            "#
            .trim(),
            r#"
region: mena
country: tr
            "#
            .trim(),
            r#"
region: ap
            "#
            .trim(),
        ];

        let input = r#"
region: {=REGION}
country: {=COUNTRY}
{+endpoint: $COUNTRY.$REGION.app.local}
"#
        .trim();

        let result = replace(&files, input);

        assert_eq!(
            result,
            vec![
                r#"
region: ap
country: jp
endpoint: jp.ap.app.local
            "#
                .trim(),
                r#"
region: eu
country: de
endpoint: de.eu.app.local
            "#
                .trim(),
                r#"
region: mena
country: tr
endpoint: tr.mena.app.local
            "#
                .trim(),
                r#"
region: ap
            "#
                .trim(),
            ]
        );
    }
}
