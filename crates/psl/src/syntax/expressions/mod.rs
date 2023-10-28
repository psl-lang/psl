use winnow::{combinator::alt, Located, PResult, Parser};

use crate::ast::Expression;

use self::read::parse_read;

mod read;

pub fn parse_expression(s: &mut Located<&str>) -> PResult<Expression> {
    alt((parse_read.map(Expression::Read),)).parse_next(s)
}
