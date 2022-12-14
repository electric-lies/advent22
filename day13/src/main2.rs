use std::fs::File;
use std::io::{prelude::*, BufReader};

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{alphanumeric1 as alphanumeric, char, one_of};
use nom::character::is_alphanumeric;
use nom::combinator::{cut, map};
use nom::error::{context, ContextError, ParseError};
use nom::multi::separated_list0;
use nom::sequence::{preceded, terminated};
use nom::IResult;
#[derive(Debug)]
enum Element {
    Num(i32),
    List(Vec<Element>),
}

fn number<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Element, E> {
    map(take_while(|c| is_alphanumeric(c as u8)), |x: &str| {
        Element::Num(x.parse::<i32>().unwrap())
    })(i)
}

fn array<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Element, E> {
    map(
        context(
            "array",
            preceded(
                char('['),
                cut(terminated(separated_list0(char(','), element), char(']'))),
            ),
        ),
        Element::List,
    )(i)
}

fn element<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Element, E> {
    alt((number, array))(i)
}

fn main() {
    println!("Hello, world!");
    let file = File::open("input").expect("did not found file");
    let packet_tups = BufReader::new(file)
        .lines()
        // .take(600)
        .filter_map(|x| x.ok())
        .map(|x| element(&x))
        .tuples()
        .map(|(p1, p2)| (p1, p2));
}
