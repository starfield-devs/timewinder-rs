use nom::{Parser, bytes::complete::take_while1, combinator::opt, multi::many1, sequence::terminated};
use nom_supreme::{error::ErrorTree, parser_ext::ParserExt, tag::complete::tag};

use crate::{slices::*, traits::Parsable};

pub struct KeyTag;

pub type Key<'a> = Slices<'a, KeyTag>;

impl<'a> Parsable<'a> for Key<'a> {
    fn parser() -> impl Parser<&'a str, Self, ErrorTree<&'a str>> {
        move |input: &'a str| {
            let (input, slices) = many1(terminated(
                take_while1(|c: char| c.is_ascii_alphanumeric() || c == '-'),
                opt(tag("\r\n ")),
            ))
            .context("Keys must be [A-Za-z\\-], optionally with continuance.")
            .parse(input)?;

            Ok((input, Key::new(slices)))
        }
    }
}
