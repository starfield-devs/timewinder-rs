use std::{
    path::{Path, PathBuf},
    thread::current,
};

use anyhow::{Context, Error, Result};
use nom::{
    Err as NomErr, IResult, Parser,
    branch::alt,
    bytes::complete::{
        escaped, is_not, tag, take, take_until, take_while, take_while_m_n, take_while1,
    },
    character::complete::{anychar, char, line_ending, one_of},
    combinator::{opt, peek, recognize, verify},
    error::{Error as NomError, ErrorKind as NomErrorKind, context},
    multi::{fold_many0, many0, many1},
    sequence::{delimited, preceded, terminated},
};

use crate::{tokens::*, traits::*};

// fn parameter(input: &str) -> IResult<&str, (&str, &str, bool)> {
//     let (input, value) = alt((
//         context(
//             "Failed to take escaped-string parameter value.",
//             delimited(
//                 char('"'),
//                 escaped(is_not("\\\""), '\\', one_of(r#""n\"#)),
//                 char('"'),
//             ),
//         ),
//         context(
//             "Failed to take normal parameter value.",
//             terminated(take_while1(|c| c != ':' && c != ';'), one_of(":;")),
//         ),
//     ))
//     .parse(input)?;

//     Ok((input, (key, value)))
// }

pub fn ical_parse<'a>(input: &'a str) -> IResult<&'a str, Component<'a>> {
    let (input, key) = Component::parser().parse(input)?;

    // println!("{:?}", (key.equals_str("BEGIN")));

    // let (input, delim) = one_of(";:=").parse(input)?;

    // let

    Ok((input, key))
}

// fn key(input: &str) -> IResult<&str, ParseToken> {
//     let (input, slice) = take_while1(|c: char| c.is_ascii_alphanumeric() || "\r\n -".contains(c)).parse(input)?;
//     let (input, follows) = take_while_m_n(1, 1, |c| ";:=".contains(c)).parse(input)?;

//     Ok((input, ParseToken {slice, follows}))
// }

// fn param_value(input)

// fn value(input: &str) -> IResult<&str, &str> {
//     recognize((
//         context(
//             "Failed to take iCal property value initial `\\r\\n`-terminated line.",
//             terminated(take_until("\r\n"), line_ending),
//         ),
//         context(
//             "Failed to take iCal property value `\\r\\n`-terminated continuation line(s).",
//             many0(preceded(
//                 alt((char(' '), char('\t'))),
//                 terminated(take_until("\r\n"), line_ending),
//             )),
//         ),
//     ))
//     .parse(input)
// }

// fn key_value(input: &str) -> IResult<&str, (&str, &str)> {
//     let (input, key) = key.parse(input)?;
//     let (input, value) = value.parse(input)?;

//     Ok((input, (key, value)))
// }

// fn parameter(input: &str) -> IResult<&str, (&str, &str, bool)> {
//     let (input, value) = alt((
//         context(
//             "Failed to take escaped-string parameter value.",
//             delimited(
//                 char('"'),
//                 escaped(is_not("\\\""), '\\', one_of(r#""n\"#)),
//                 char('"'),
//             ),
//         ),
//         context(
//             "Failed to take normal parameter value.",
//             terminated(take_while1(|c| c != ':' && c != ';'), one_of(":;")),
//         ),
//     ))
//     .parse(input)?;

//     Ok((input, (key, value)))
// }

//     let parameters: Vec<(&str, &str)> = Vec::new();
//     let properties: Vec<(&str, Vec<&str>)> = Vec::new();
//     let children: Vec<ICalNode> = Vec::new();

//     loop {
//         let (in1, key) = key.parse(input)?;
//         if let Ok((_, key)) = peek(char(':')).parse(input) {

//         }
//     }

//     // let (_, _) = context(
//     //     "iCal files must start with a BEGIN block!",
//     //     verify(key, |k: &str| k == "BEGIN"),
//     // )
//     // .parse(input)?;

//     // let (input, properties): (&str, Vec<(&str, &str)>) = many0(key_value).parse(input)?;
//     // println!("{:#?}", properties);

//     let

//     Ok((input, ()))
// }
