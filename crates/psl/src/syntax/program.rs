use winnow::Parser;
use winnow::{combinator::repeat, Located, PResult};

use crate::ast::{Item, Program};

use super::item::parse_item;

pub fn parse_program(s: &mut Located<&str>) -> PResult<Program> {
    repeat(0.., parse_item)
        .map(|items: Vec<Item>| Program { items })
        .parse_next(s)
}
