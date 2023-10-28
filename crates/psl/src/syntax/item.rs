use winnow::{
    combinator::{alt, terminated},
    Located, PResult, Parser,
};

use crate::ast::Item;

use super::{
    declarations::parse_declaration, fragments::separator::parse_separator,
    statements::parse_statement,
};

pub fn parse_item(s: &mut Located<&str>) -> PResult<Item> {
    terminated(
        alt((
            parse_declaration.map(Item::Declaration),
            parse_statement.map(Item::Statement),
        )),
        parse_separator,
    )
    .parse_next(s)
}
