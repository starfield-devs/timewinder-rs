use nom::{
    Parser,
    branch::alt,
    bytes::complete::{is_not, tag, take_while1},
    character::complete::char,
    combinator::opt,
    error::context,
    multi::many1,
    sequence::{delimited, terminated},
};

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
    fn parser() -> impl Parser<&'a str, Output = Self, Error = nom::error::Error<&'a str>> {
        move |input: &'a str| {
            let (input, key) = Key::parser().parse(input)?;

            let (input, _) = context(
                "Parameter key and value must be separated by `=`",
                terminated(char('='), opt(tag("\r\n"))),
            )
            .parse(input)?;

            let (input, value_slices) = alt((
                context(
                    "Failed to consume quoted parameter value.",
                    delimited(
                        char('"'),
                        many1(terminated(is_not("\"\r\n"), opt(tag("\r\n ")))),
                        char('"'),
                    ),
                ),
                context(
                    "Failed to consume normal parameter value.",
                    many1(terminated(take_while1(|c| c != ':' && c != ';'), opt(tag("\r\n ")))),
                ),
            ))
            .parse(input)?;

            Ok((
                input,
                Parameter {
                    key,
                    value: Value::new(value_slices),
                },
            ))
        }
    }
}
