use nom::{
    Parser,
    bytes::complete::{tag, take_until},
    combinator::verify,
    error::{Error, context},
    sequence::terminated,
};

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
    fn parser() -> impl Parser<&'a str, Output = Self, Error = Error<&'a str>> {
        move |input: &'a str| {
            let (input, key) = context(
                "So-called properties with keys `BEGIN` or `END` are actually components",
                verify(Key::parser(), |k: &Key| !k.equals("BEGIN") && !k.equals("END")),
            )
            .parse(input)?;

            // // TOOD: handle parameters vs going straight to values
            // let (input, parameters) = many0(Parameter::parser()).parse(input)?;
            // let (input, _) = context(
            //     "Properties must have `:` in `KEY:VALUE`",
            //     terminated(char(':'), opt(tag("\r\n"))),
            // )
            // .parse(input)?;
            // // DELETEME NOTE: This bypasses parameters for testing.
            let (input, _) = terminated(take_until(":"), tag(":")).parse(input)?;

            let (input, value) = Value::parser().parse(input)?;

            Ok((
                input,
                Property {
                    key,
                    value,
                    ..Default::default()
                },
            ))
        }
    }
}
