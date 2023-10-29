use winnow::combinator::{delimited, opt};
use winnow::Parser;
use winnow::{combinator::repeat, Located, PResult};

use crate::ast::{Item, Program};

use super::fragments::separator::parse_separator;
use super::item::parse_item;

pub fn parse_program(s: &mut Located<&str>) -> PResult<Program> {
    delimited(
        opt(parse_separator),
        repeat(0.., parse_item),
        opt(parse_separator),
    )
    .map(|items: Vec<Item>| Program { items })
    .parse_next(s)
}
