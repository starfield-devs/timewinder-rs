use nom::{
    Parser,
    bytes::complete::take_until,
    character::complete::one_of,
    multi::many0,
    sequence::{preceded, terminated},
};
use nom_supreme::{error::ErrorTree, parser_ext::ParserExt, tag::complete::tag};

use crate::{slices::*, traits::Parsable};

pub struct ValueTag;

pub type Value<'a> = Slices<'a, ValueTag>;

impl<'a> Parsable<'a> for Value<'a> {
    fn parser() -> impl Parser<&'a str, Self, ErrorTree<&'a str>> {
        move |input: &'a str| {
            let (input, first_line) = terminated(take_until("\r\n"), tag("\r\n"))
                .context("Value line must end with CRLF")
                .parse(input)?;

            let (input, mut continuations) =
                many0(preceded(one_of(" \t"), terminated(take_until("\r\n"), tag("\r\n"))))
                    .context("Value continuation lines must start with space or tab")
                    .parse(input)?;

            continuations.insert(0, first_line);

            Ok((input, Value::new(continuations)))
        }
    }
}
