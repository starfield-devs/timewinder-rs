mod slices;
pub mod tokens;
pub mod traits;

#[cfg(test)]
mod tests {
    use std::path::Path;

    use anyhow::Result;
    use nom::Parser;
    use traits::*;

    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn starter_nom() -> Result<()> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/icalnet/journal/JOURNAL1.ics");
        let input = std::fs::read_to_string(path)?;
        let (_, parsed) = match tokens::Component::parser().parse(&input) {
            Ok((rest, key)) => (rest, key),
            Err(e) => anyhow::bail!("Parse error: {:?}", e),
        };
        println!("{:#?}", parsed);
        // match ical::ical_parse(&input) {
        //     Ok((residual, tree)) => {
        //         println!("Residual: {}", residual);
        //         println!("Tree: {:#?}", tree);
        //     }
        //     Err(err) => println!("Parse error: {:?}", err),
        // }

        Ok(())
    }
}
