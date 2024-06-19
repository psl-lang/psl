use winnow::{
    combinator::{alt, opt},
    Located, PResult, Parser,
};

use crate::ast::{Expression, PostfixOperatorExpression};

use super::{
    literal::parse_literal, name::parse_name, postfix_operator::parse_postfix_operator,
    r#if::parse_if, r#return::parse_return, read::parse_read,
};

pub fn parse_simple_expression(s: &mut Located<&str>) -> PResult<Expression> {
    (
        alt((
            parse_literal.map(Expression::Literal),
            parse_read.map(Expression::Read),
            parse_name.map(Expression::Name),
            parse_if.map(Expression::If),
            parse_return.map(Expression::Return),
        )),
        opt(parse_postfix_operator),
    )
        .map(|(expr, postfix)| {
            if let Some(operator) = postfix {
                Expression::PostfixOperator(PostfixOperatorExpression {
                    expr: Box::new(expr),
                    operator,
                })
            } else {
                expr
            }
        })
        .parse_next(s)
}
