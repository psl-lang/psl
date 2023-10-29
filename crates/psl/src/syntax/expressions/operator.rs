use winnow::{
    combinator::{alt, opt},
    error::ContextError,
    Located, PResult, Parser,
};

use crate::ast::{BinaryOperator, BinaryOperatorExpression, Expression, TokenKind};

use super::simple::parse_simple_expression;

pub fn parse_operator(s: &mut Located<&str>) -> PResult<Expression> {
    parse_addsub_operator.parse_next(s)
}

pub fn parse_addsub_operator(s: &mut Located<&str>) -> PResult<Expression> {
    binary_operator(
        alt((
            TokenKind::PunctuationPlusSign.map(|_| BinaryOperator::Add),
            TokenKind::PunctuationHyphenMinus.map(|_| BinaryOperator::Subtract),
        )),
        parse_muldiv_operator,
    )
    .parse_next(s)
}

pub fn parse_muldiv_operator(s: &mut Located<&str>) -> PResult<Expression> {
    binary_operator(
        alt((
            TokenKind::PunctuationAsterisk.map(|_| BinaryOperator::Multiply),
            TokenKind::PunctuationSolidus.map(|_| BinaryOperator::Divide),
        )),
        parse_simple_expression,
    )
    .parse_next(s)
}

fn binary_operator<'a>(
    operator: impl Parser<Located<&'a str>, BinaryOperator, ContextError>,
    parser: impl Parser<Located<&'a str>, Expression, ContextError> + Clone,
) -> impl Parser<Located<&'a str>, Expression, ContextError> {
    (
        parser.clone(),
        opt((
            opt(TokenKind::WhitespaceHorizontal),
            operator,
            opt(TokenKind::WhitespaceHorizontal),
            parser,
        )),
    )
        .map(|(lhs, rhs)| match rhs {
            Some((_, operator, _, rhs)) => Expression::BinaryOperator(BinaryOperatorExpression {
                lhs: Box::new(lhs),
                operator,
                rhs: Box::new(rhs),
            }),
            None => lhs,
        })
}
