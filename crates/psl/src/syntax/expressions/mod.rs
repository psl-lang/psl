use winnow::{Located, PResult, Parser};

use crate::ast::Expression;

use self::operator::parse_operator;

mod name;
mod operator;
mod read;
mod simple;

pub fn parse_expression(s: &mut Located<&str>) -> PResult<Expression> {
    parse_operator.parse_next(s)
}
