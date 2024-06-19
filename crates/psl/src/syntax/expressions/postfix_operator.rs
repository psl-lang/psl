use winnow::{
    combinator::{alt, delimited, opt, separated1},
    Located, PResult, Parser,
};

use crate::ast::{PostfixOperator, TokenKind};

use super::parse_expression;

pub fn parse_postfix_operator(s: &mut Located<&str>) -> PResult<PostfixOperator> {
    alt((parse_invoke_operator, parse_index_operator)).parse_next(s)
}

fn parse_invoke_operator(s: &mut Located<&str>) -> PResult<PostfixOperator> {
    delimited(
        TokenKind::PunctuationLeftParenthesis,
        opt(separated1(
            parse_expression,
            (
                TokenKind::PunctuationComma,
                opt(TokenKind::WhitespaceHorizontal),
            ),
        )),
        TokenKind::PunctuationRightParenthesis,
    )
    .map(|arguments| PostfixOperator::Invoke(arguments.unwrap_or(Vec::new())))
    .parse_next(s)
}

fn parse_index_operator(s: &mut Located<&str>) -> PResult<PostfixOperator> {
    delimited(
        TokenKind::PunctuationLeftSquareBracket,
        opt(separated1(
            parse_expression,
            (
                TokenKind::PunctuationComma,
                opt(TokenKind::WhitespaceHorizontal),
            ),
        )),
        TokenKind::PunctuationRightSquareBracket,
    )
    .map(|arguments| PostfixOperator::Invoke(arguments.unwrap_or(Vec::new())))
    .parse_next(s)
}
