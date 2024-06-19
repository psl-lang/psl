use winnow::{Located, PResult, Parser};

use crate::ast::Expression;

use binary_operator::parse_binary_operator;
pub use name::parse_name;

mod binary_operator;
mod r#if;
mod literal;
mod name;
mod postfix_operator;
mod read;
mod r#return;
mod simple;

pub fn parse_expression(s: &mut Located<&str>) -> PResult<Expression> {
    parse_binary_operator.parse_next(s)
}
