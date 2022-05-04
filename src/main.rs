use std::env;
use std::error::Error;
use std::fs;

use glob::glob;

mod eval;
mod match_;
mod pattern;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_, input, files] => {
            let files = glob(files)?
                .filter_map(Result::ok)
                .filter(|p| p.is_file())
                .filter_map(|p| Some((p.clone(), fs::read_to_string(p).ok()?)))
                .collect::<Vec<_>>();

            let replacements = match replace(
                &files.iter().map(|(_, f)| f.as_str()).collect::<Vec<_>>(),
                input.trim(),
            ) {
                Some(it) => it,
                None => return Ok(()),
            };

            for ((path, content), new_content) in files.iter().zip(replacements) {
                if content == &new_content {
                    continue;
                }
                fs::write(path, new_content)?;
                println!("{}", path.display());
            }
        }
        _ => print_help(),
    }

    Ok(())
}

fn print_help() {
    println!("usage: bulke <input> <glob>");
}

fn replace(files: &[&str], input: &str) -> Option<Vec<String>> {
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
