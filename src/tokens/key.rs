use std::fmt::{Debug, Formatter, Result};

use nom::{
    Parser,
    bytes::complete::{tag, take_while1},
    combinator::opt,
    multi::many1,
    sequence::terminated,
};

use crate::traits::Parsable;

#[derive(Default, PartialEq, Eq)]
pub struct Key<'a> {
    pub slices: Vec<&'a str>,
}

impl<'a> Debug for Key<'a> {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> Result {
        write!(f, "{}", self.slices.concat())
    }
}

impl<'a> AsRef<[&'a str]> for Key<'a> {
    fn as_ref(&self) -> &[&'a str] {
        &self.slices
    }
}

impl<'a> Parsable<'a> for Key<'a> {
    fn parser() -> impl Parser<&'a str, Output = Self, Error = nom::error::Error<&'a str>> {
        move |input: &'a str| {
            let (input, slices) = many1(terminated(
                take_while1(|c: char| c.is_ascii_alphanumeric() || c == '-'),
                opt(tag("\r\n ")),
            ))
            .parse(input)?;

            Ok((input, Key { slices }))
        }
    }
}
