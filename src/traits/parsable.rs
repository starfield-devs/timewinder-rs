use nom::{Parser, error::Error};

pub trait Parsable<'a>: Sized {
    fn parser() -> impl Parser<&'a str, Output = Self, Error = Error<&'a str>>;
}
