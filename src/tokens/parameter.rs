use nom::{
    Parser,
    branch::alt,
    bytes::complete::{is_not, take_while1},
    character::complete::char,
    combinator::opt,
    multi::many0,
    sequence::{delimited, terminated},
};
use nom_supreme::{error::ErrorTree, parser_ext::ParserExt, tag::complete::tag};

use crate::{
    tokens::{Key, Value},
    traits::Parsable,
};

#[derive(Debug, Default)]
pub struct Parameter<'a> {
    pub key: Key<'a>,
    pub value: Value<'a>,
}

impl<'a> Parsable<'a> for Parameter<'a> {
    fn parser() -> impl Parser<&'a str, Self, ErrorTree<&'a str>> {
        move |input: &'a str| {
            // parse the key
            let (input, key) = Key::parser().parse(input)?;

            // parse '=' separator
            let (input, _) = terminated(char('='), opt(tag("\r\n")))
                .context("Parameter key and value must be separated by '='")
                .parse(input)?;

            // parse the value until next `;` or `:` (zero-copy)
            let (input, slices) = alt((
                // quoted value
                delimited(
                    char('"'),
                    many0(terminated(
                        is_not("\"\r\n"),  // content inside quotes
                        opt(tag("\r\n ")), // handle folded continuation
                    )),
                    char('"'),
                ),
                // unquoted value
                many0(terminated(
                    take_while1(|c| c != ';' && c != ':' && c != '\r' && c != '\n'),
                    opt(tag("\r\n ")), // folded lines
                )),
            ))
            .parse(input)?;

            Ok((
                input,
                Parameter {
                    key,
                    value: Value::new(slices),
                },
            ))
        }
    }
}
