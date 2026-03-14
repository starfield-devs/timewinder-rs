use nom::{
    Parser,
    bytes::complete::{tag, take_while1},
    combinator::opt,
    multi::many1,
    sequence::terminated,
};

use crate::{slices::*, traits::Parsable};

pub struct KeyTag;

pub type Key<'a> = SlicesWrapper<'a, KeyTag>;

impl<'a> Parsable<'a> for Key<'a> {
    fn parser() -> impl Parser<&'a str, Output = Self, Error = nom::error::Error<&'a str>> {
        move |input: &'a str| {
            let (input, slices) = many1(terminated(
                take_while1(|c: char| c.is_ascii_alphanumeric() || c == '-'),
                opt(tag("\r\n ")),
            ))
            .parse(input)?;

            Ok((input, Key::new(slices)))
        }
    }
}
