use super::pattern;

// FIXME: proper finite automata implementation.

#[derive(Debug, PartialEq, Clone)]
pub struct Pos(pub usize, pub usize);

#[derive(Debug, PartialEq)]
pub struct Match(pub Vec<(pattern::Rule, Pos)>);

fn match_rule<'a>(input: &'a str, rule: &pattern::Rule) -> Option<&'a str> {
    match rule {
        pattern::Rule::Literal(s) => {
            let len = s.len();
            if len > input.len() {
                return None;
            }
            if &input[..len] != s {
                return None;
            }
            Some(&input[len..])
        }
        pattern::Rule::LineBreak => {
            if input.is_empty() {
                return None;
            }

            let mut chars = input.chars();
            if chars.next() != Some('\n') {
                return None;
            }

            if input.len() > 1 && chars.next() == Some('\r') {
                Some(&input[2..])
            } else {
                Some(&input[1..])
            }
        }
        _ => unreachable!(),
    }
}

fn match_rules<'a>(mut input: &'a str, rules: &'a [pattern::Rule]) -> Option<Match> {
    let mut pos = 0usize;
    let mut matches_ = rules
        .iter()
        .map(|r| (r.clone(), Pos(0, 0)))
        .collect::<Vec<_>>();
    'main: for i in 0..rules.len() {
        let r = &rules[i];
        let is_placeholder = r.is_placeholder();
        if !is_placeholder {
            match match_rule(input, r) {
                Some(rest) => {
                    let delta = input.len() - rest.len();
                    matches_[i] = (matches_[i].0.clone(), Pos(pos, pos + delta));
                    pos += delta;
                    input = rest;
                    continue;
                }
                None => return None,
            }
        }

        if i == rules.len() {
            return None;
        }

        for j in 0..input.len() {
            let rest = &input[j..];
            if match_rules(rest, &rules[i + 1..]).is_some() {
                let delta = input.len() - rest.len();
                matches_[i] = (matches_[i].0.clone(), Pos(pos, pos + delta));
                pos += delta;
                input = rest;
                continue 'main;
            }
        }
        return None;
    }
    Some(Match(matches_))
}

pub fn match_(input: &str, pattern: &pattern::Pattern) -> Option<Match> {
    let rules = &pattern.0;
    for i in 0..input.len() {
        if let Some(m) = match_rules(&input[i..], rules) {
            return Some(Match(
                m.0.into_iter()
                    .map(|(r, p)| (r, Pos(p.0 + i, p.1 + i)))
                    .collect(),
            ));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matching() {
        assert_eq!(
            match_(
                "\
qux: baz
foo: bar
bar: 42
qux: foo
",
                &pattern::parse(
                    "\
foo: bar
bar: {=BAR}
{+baz: qux}
",
                )
                .unwrap(),
            ),
            Some(Match(vec![
                (pattern::Rule::Literal("foo: bar".to_string()), Pos(9, 17)),
                (pattern::Rule::LineBreak, Pos(17, 18)),
                (pattern::Rule::Literal("bar: ".to_string()), Pos(18, 23)),
                (pattern::Rule::Capture("BAR".to_string()), Pos(23, 25)),
                (pattern::Rule::LineBreak, Pos(25, 26)),
                (pattern::Rule::Add("baz: qux".to_string()), Pos(26, 34)),
                (pattern::Rule::LineBreak, Pos(34, 35)),
            ]))
        );
    }

    #[test]
    fn not_matching() {
        let pattern = pattern::Pattern(vec![
            pattern::Rule::Literal("foo: bar".to_string()),
            pattern::Rule::LineBreak,
        ]);
        assert_eq!(match_("foo: bar", &pattern), None);
    }
}
