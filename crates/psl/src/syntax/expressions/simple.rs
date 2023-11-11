use winnow::{combinator::alt, Located, PResult, Parser};

use crate::ast::Expression;

use super::{block::parse_block, name::parse_name, r#if::parse_if, read::parse_read};

pub fn parse_simple_expression(s: &mut Located<&str>) -> PResult<Expression> {
    alt((
        parse_read.map(Expression::Read),
        parse_name.map(Expression::Name),
        parse_if.map(Expression::If),
        parse_block.map(Expression::Block),
    ))
    .parse_next(s)
}
