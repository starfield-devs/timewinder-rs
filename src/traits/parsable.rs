use nom::Parser;
use nom_supreme::error::ErrorTree;

pub trait Parsable<'a>: Sized {
    fn parser() -> impl Parser<&'a str, Self, ErrorTree<&'a str>>;
}
