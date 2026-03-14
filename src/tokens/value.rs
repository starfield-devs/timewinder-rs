use nom::{
    Parser,
    bytes::complete::{tag, take_until},
    character::complete::one_of,
    error::Error,
    multi::many0,
    sequence::{preceded, terminated},
};

use crate::{slices::*, traits::Parsable};

pub struct ValueTag;

pub type Value<'a> = Slices<'a, ValueTag>;

impl<'a> Parsable<'a> for Value<'a> {
    fn parser() -> impl Parser<&'a str, Output = Self, Error = Error<&'a str>> {
        move |input: &'a str| {
            let (input, first_line) = terminated(take_until("\r\n"), tag("\r\n")).parse(input)?;
            let (input, mut continuations) =
                many0(preceded(one_of(" \t"), terminated(take_until("\r\n"), tag("\r\n")))).parse(input)?;

            continuations.insert(0, first_line);

            Ok((input, Value::new(continuations)))
        }
    }
}
