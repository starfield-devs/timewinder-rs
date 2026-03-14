use std::fmt::{Debug, Formatter, Result};

use nom::{
    Parser,
    bytes::complete::{tag, take_until},
    character::complete::one_of,
    multi::many0,
    sequence::{preceded, terminated},
};

use crate::traits::Parsable;

#[derive(Default, PartialEq, Eq)]
pub struct Value<'a> {
    pub slices: Vec<&'a str>,
}

impl<'a> Debug for Value<'a> {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> Result {
        write!(f, "{}", self.slices.concat())
    }
}

impl<'a> AsRef<[&'a str]> for Value<'a> {
    fn as_ref(&self) -> &[&'a str] {
        &self.slices
    }
}

impl<'a> Parsable<'a> for Value<'a> {
    fn parser() -> impl Parser<&'a str, Output = Self, Error = nom::error::Error<&'a str>> {
        move |input: &'a str| {
            let (input, first_line) = terminated(take_until("\r\n"), tag("\r\n")).parse(input)?;
            let (input, mut continuations) =
                many0(preceded(one_of(" \t"), terminated(take_until("\r\n"), tag("\r\n")))).parse(input)?;

            continuations.insert(0, first_line);

            Ok((input, Value { slices: continuations }))
        }
    }
}
