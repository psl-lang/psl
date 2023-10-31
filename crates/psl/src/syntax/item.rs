use winnow::{combinator::alt, Located, PResult, Parser};

use crate::ast::Item;

use super::{declarations::parse_declaration, statements::parse_statement};

pub fn parse_item(s: &mut Located<&str>) -> PResult<Item> {
    alt((
        parse_declaration.map(Item::Declaration),
        parse_statement.map(Item::Statement),
    ))
    .parse_next(s)
}
