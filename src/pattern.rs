#[derive(Debug, PartialEq, Clone)]
pub enum Rule {
    Literal(String),
    LineBreak,
    Capture(String),
    Add(String),
    Remove(String),
}

impl Rule {
    pub fn is_placeholder(&self) -> bool {
        matches!(self, Rule::Capture(_) | Rule::Add(_) | Rule::Remove(_))
    }
}

#[derive(Debug, PartialEq)]
pub struct Pattern(pub Vec<Rule>);

pub fn parse(input: &str) -> Option<Pattern> {
    let mut rules = Vec::new();
    let mut chars = input.chars().peekable();
    loop {
        let mut line_break = false;

        let mut literal = chars
            .by_ref()
            .take_while(|c| {
                if *c == '\n' {
                    line_break = true;
                    false
                } else {
                    *c != '{'
                }
            })
            .collect::<String>();
        let literal_len = literal.len();
        let has_literal = literal_len > 0;
        if has_literal {
            if line_break && literal.as_bytes()[literal_len - 1] == b'\r' {
                literal.pop();
            }
            rules.push(Rule::Literal(literal));
        }
        if line_break {
            rules.push(Rule::LineBreak);
            continue;
        }

        let ty = chars.next();
        let mut expr = chars
            .by_ref()
            .take_while(|c| {
                if *c == '\n' {
                    line_break = true;
                    false
                } else {
                    *c != '}'
                }
            })
            .collect::<String>();
        let expr_len = expr.len();
        let has_expr = expr_len > 0;
        if has_expr {
            if line_break && expr.as_bytes()[expr_len - 1] == b'\r' {
                expr.pop();
            }

            match ty {
                Some('=') => {
                    rules.push(Rule::Capture(expr));
                }
                Some('+') => {
                    rules.push(Rule::Add(expr));
                }
                Some('-') => {
                    rules.push(Rule::Remove(expr));
                }
                _ => {
                    return None;
                }
            }
        }
        if line_break {
            rules.push(Rule::LineBreak);
            continue;
        }

        if !has_literal && !has_expr {
            break;
        }
    }

    Some(Pattern(rules))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(
            parse(
                "\
foo: bar
bar: {=BAR}
{-answer: 42}
{+baz: qux}
",
            )
            .unwrap(),
            Pattern(vec![
                Rule::Literal("foo: bar".to_string()),
                Rule::LineBreak,
                Rule::Literal("bar: ".to_string()),
                Rule::Capture("BAR".to_string()),
                Rule::LineBreak,
                Rule::Remove("answer: 42".to_string()),
                Rule::LineBreak,
                Rule::Add("baz: qux".to_string()),
                Rule::LineBreak,
            ])
        );
    }
}
