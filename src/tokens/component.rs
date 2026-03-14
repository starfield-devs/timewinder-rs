use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::char,
    combinator::{opt, verify},
    error::context,
    multi::many0,
    sequence::terminated,
};

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
    fn parser() -> impl Parser<&'a str, Output = Self, Error = nom::error::Error<&'a str>> {
        move |input: &'a str| {
            let (input, _) = context(
                "Components must begin with a `BEGIN`",
                verify(Key::parser(), |k: &Key| k.equals_str("BEGIN")),
            )
            .parse(input)?;

            let (input, _) = context(
                "Components must have `:` in `BEGIN:NAME`",
                terminated(char(':'), opt(tag("\r\n"))),
            )
            .parse(input)?;

            let (input, name) = context(
                "Components names must start with `V`",
                terminated(
                    verify(Key::parser(), |k: &Key| k.starts_with_str("V")),
                    opt(tag("\r\n")),
                ),
            )
            .parse(input)?;

            let (input, properties) = many0(terminated(Property::parser(), opt(tag("\r\n")))).parse(input)?;
            let (input, components) = many0(terminated(Component::parser(), opt(tag("\r\n")))).parse(input)?;

            let (input, _) = context(
                "Components must end with an `END`",
                verify(Key::parser(), |k: &Key| k.equals_str("END")),
            )
            .parse(input)?;

            let (input, _) = context(
                "Components must have `:` in `END:NAME`",
                terminated(char(':'), opt(tag("\r\n"))),
            )
            .parse(input)?;

            let (input, _) = context(
                "Components names must start with `V`",
                terminated(
                    verify(Key::parser(), |k: &Key| k.equals_slices(name.slices())),
                    opt(tag("\r\n")),
                ),
            )
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
