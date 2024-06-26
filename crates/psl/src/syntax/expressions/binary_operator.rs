use winnow::{
    combinator::{alt, opt, repeat},
    error::ContextError,
    Located, PResult, Parser,
};

use crate::ast::{BinaryOperator, BinaryOperatorExpression, Expression, TokenKind};

use super::simple::parse_simple_expression;

pub fn parse_binary_operator(s: &mut Located<&str>) -> PResult<Expression> {
    parse_or_operator.parse_next(s)
}

pub fn parse_or_operator(s: &mut Located<&str>) -> PResult<Expression> {
    binary_operator(
        (
            TokenKind::PunctuationVerticalLine,
            TokenKind::PunctuationVerticalLine,
        )
            .map(|_| BinaryOperator::LogicalOr),
        parse_and_operator,
    )
    .parse_next(s)
}

pub fn parse_and_operator(s: &mut Located<&str>) -> PResult<Expression> {
    binary_operator(
        (
            TokenKind::PunctuationAmpersand,
            TokenKind::PunctuationAmpersand,
        )
            .map(|_| BinaryOperator::LogiacalAnd),
        parse_comparison_operator,
    )
    .parse_next(s)
}

pub fn parse_comparison_operator(s: &mut Located<&str>) -> PResult<Expression> {
    binary_operator(
        alt((
            (
                TokenKind::PunctuationLessThanSign,
                TokenKind::PunctuationEqualsSign,
            )
                .map(|_| BinaryOperator::LessEqual),
            (
                TokenKind::PunctuationGreaterThanSign,
                TokenKind::PunctuationEqualsSign,
            )
                .map(|_| BinaryOperator::GreaterEqual),
            (
                TokenKind::PunctuationExclamationMark,
                TokenKind::PunctuationEqualsSign,
            )
                .map(|_| BinaryOperator::NotEqual),
            (
                TokenKind::PunctuationEqualsSign,
                TokenKind::PunctuationEqualsSign,
            )
                .map(|_| BinaryOperator::Equal),
            TokenKind::PunctuationLessThanSign.map(|_| BinaryOperator::Less),
            TokenKind::PunctuationGreaterThanSign.map(|_| BinaryOperator::Greater),
        )),
        parse_addsub_operator,
    )
    .parse_next(s)
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
            TokenKind::PunctuationPercentSign.map(|_| BinaryOperator::Modulus),
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
        repeat::<_, _, Vec<_>, _, _>(
            ..,
            (
                opt(TokenKind::WhitespaceHorizontal),
                operator,
                opt(TokenKind::WhitespaceHorizontal),
                parser,
            )
                .map(|(_, operator, _, rhs)| (operator, rhs)),
        ),
    )
        .map(|(mut lhs, seq)| {
            for (operator, rhs) in seq {
                lhs = Expression::BinaryOperator(BinaryOperatorExpression {
                    lhs: Box::new(lhs),
                    operator,
                    rhs: Box::new(rhs),
                });
            }

            lhs
        })
}
