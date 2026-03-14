use nom::{
    Parser,
    character::complete::one_of,
    combinator::{opt, verify},
    sequence::terminated,
};
use nom_supreme::{error::ErrorTree, parser_ext::ParserExt, tag::complete::tag};

use crate::{
    tokens::{Key, Parameter, Value},
    traits::{Parsable, SlicesOps},
};

#[derive(Debug, Default)]
pub struct Property<'a> {
    pub key: Key<'a>,
    pub value: Value<'a>,
    pub parameters: Vec<Parameter<'a>>,
}

impl<'a> Parsable<'a> for Property<'a> {
    fn parser() -> impl Parser<&'a str, Self, ErrorTree<&'a str>> {
        move |input: &'a str| {
            let (mut input, key) = verify(terminated(Key::parser(), opt(tag("\r\n"))), |k: &Key| {
                !k.equals("BEGIN") && !k.equals("END")
            })
            .context("")
            .parse(input)?;

            let mut parameters = Vec::new();

            while let Ok((i, delim)) = one_of::<&str, _, ErrorTree<&str>>(";:").parse(input) {
                input = i;

                match delim {
                    ';' => {
                        let (i, param) = Parameter::parser().parse(input)?;
                        parameters.push(param);
                        input = i;
                    }
                    ':' => break,
                    _ => unreachable!("one_of guarantees either ';' or ':'"),
                }
            }

            // parse the value after the colon
            let (input, value) = Value::parser().parse(input)?;

            Ok((input, Property { key, value, parameters }))
        }
    }
}
