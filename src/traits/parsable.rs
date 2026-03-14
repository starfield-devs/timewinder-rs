use nom::Parser;

pub trait Parsable<'a>: Sized {
    fn parser() -> impl Parser<&'a str, Output = Self, Error = nom::error::Error<&'a str>>;
}
