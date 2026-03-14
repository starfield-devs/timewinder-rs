mod slices;
pub mod tokens;
pub mod traits;

#[cfg(test)]
mod tests {
    use std::path::Path;

    use anyhow::Result;
    use nom_supreme::final_parser::final_parser;
    use traits::*;

    use super::*;

    fn print_error(
        input: &str,
        err: nom_supreme::error::ErrorTree<&str>,
    ) {
        println!("\n---[ PARSE ERROR ]---\n");

        println!("{:#?}", err);

        if let nom_supreme::error::ErrorTree::Base { location, .. } = &err {
            let offset = input.len() - location.len();

            let line = input[..offset].lines().count();
            let col = input[..offset].lines().last().map(|l| l.len()).unwrap_or(0);

            let line_text = input.lines().nth(line).unwrap_or("");

            println!("\nLocation: line {}, column {}", line + 1, col + 1);
            println!("{}", line_text);
            println!("{:>width$}^", "", width = col);
        }
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn starter_nom() -> Result<()> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/icalnet/journal/JOURNAL1.ics");
        let input = std::fs::read_to_string(path)?;
        let mut parser = final_parser(tokens::Component::parser());

        match parser(&input) {
            Ok(parsed) => println!("{:#?}", parsed),
            Err(e) => {
                print_error(&input, e);
                panic!("Parse failed");
            }
        }

        Ok(())
    }
}
