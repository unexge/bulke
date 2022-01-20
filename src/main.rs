mod eval;
mod match_;
mod pattern;

fn main() {}

pub fn replace(files: &[&str], input: &str) -> Option<Vec<String>> {
    let pattern = pattern::parse(input)?;
    let res = files
        .iter()
        .map(|f| match match_::match_(f, &pattern) {
            Some(m) => eval::eval(f, &m),
            None => f.to_string(),
        })
        .collect::<Vec<_>>();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let files = vec![
            r#"
port: 8080
region: ap
country: jp
concurrency: 2
"#
            .trim(),
            r#"
port: 8080
region: eu
country: de
lifetime: 3600
            "#
            .trim(),
            r#"
region: mena
country: tr
lifetime: 3600
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

        let result = replace(&files, input).unwrap();

        assert_eq!(
            result,
            vec![
                r#"
port: 8080
region: ap
country: jp
endpoint: jp.ap.app.local
concurrency: 2
            "#
                .trim(),
                r#"
port: 8080
region: eu
country: de
endpoint: de.eu.app.local
lifetime: 3600
            "#
                .trim(),
                r#"
region: mena
country: tr
endpoint: tr.mena.app.local
lifetime: 3600
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
