use winnow::{combinator::alt, Located, PResult, Parser};

use crate::ast::Expression;

use super::{name::parse_name, read::parse_read};

pub fn parse_simple_expression(s: &mut Located<&str>) -> PResult<Expression> {
    alt((
        parse_read.map(Expression::Read),
        parse_name.map(Expression::Name),
    ))
    .parse_next(s)
}
