use winnow::{Located, PResult, Parser};

use crate::ast::Expression;

pub use name::parse_name;
use operator::parse_operator;

mod r#if;
mod name;
mod operator;
mod read;
mod simple;

pub fn parse_expression(s: &mut Located<&str>) -> PResult<Expression> {
    parse_operator.parse_next(s)
}
