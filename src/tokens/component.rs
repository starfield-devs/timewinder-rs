use nom::{
    Parser,
    character::complete::char,
    combinator::{opt, peek, verify},
    sequence::terminated,
};
use nom_supreme::{error::ErrorTree, parser_ext::ParserExt, tag::complete::tag};

use crate::{
    tokens::{Key, Property},
    traits::{Parsable, SlicesOps},
};

#[derive(Debug, Default)]
pub struct Component<'a> {
    pub name: Key<'a>,
    pub properties: Vec<Property<'a>>,
    pub components: Vec<Component<'a>>,
}

impl<'a> Parsable<'a> for Component<'a> {
    fn parser() -> impl Parser<&'a str, Self, ErrorTree<&'a str>> {
        move |input: &'a str| {
            let (input, _) = verify(Key::parser(), |k: &Key| k.equals("BEGIN"))
                .context("Components must be introduced with BEGIN:<name>.")
                .parse(input)?;

            let (input, _) = terminated(char(':'), opt(tag("\r\n")))
                .context("Component BEGIN and <name> must be separated by a colon.")
                .parse(input)?;

            let (input, name) = terminated(verify(Key::parser(), |k: &Key| k.starts_with("V")), opt(tag("\r\n")))
                .context("Component <name>s must begin with V.")
                .parse(input)?;

            let mut input = input;
            let mut properties = Vec::new();
            let mut components = Vec::new();

            while let Ok((_, next_key)) = peek(Key::parser()).parse(input) {
                if next_key.equals("END") {
                    break;
                }

                if next_key.equals("BEGIN") {
                    let (i, component) = Component::parser().parse(input)?;
                    components.push(component);
                    input = i;
                } else {
                    let (i, property) = Property::parser().parse(input)?;
                    properties.push(property);
                    input = i;
                }
            }

            let (input, _) = verify(Key::parser(), |k: &Key| k.equals("END"))
                .context("Components must be dismissed with END:<name>.")
                .parse(input)?;

            let (input, _) = terminated(char(':'), opt(tag("\r\n")))
                .context("Components END and <name> must be separated by a colon.")
                .parse(input)?;

            let (input, _) = terminated(verify(Key::parser(), |k: &Key| k.equals(&name)), opt(tag("\r\n")))
                .context("Component names must match in their ENDs.")
                .parse(input)?;

            Ok((
                input,
                Component {
                    name,
                    properties,
                    components,
                },
            ))
        }
    }
}
