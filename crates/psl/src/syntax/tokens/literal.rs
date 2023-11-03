use winnow::{
    combinator::{alt, preceded},
    token::take_while,
    Located, PResult, Parser,
};

use crate::ast::{Token, TokenKind};

use super::token;

pub fn parse_literal(s: &mut Located<&str>) -> PResult<Token> {
    parse_integer_literal.parse_next(s)
}

pub fn parse_integer_literal(s: &mut Located<&str>) -> PResult<Token> {
    alt((parse_hexadecimal, parse_binary, parse_decimal)).parse_next(s)
}

pub fn parse_decimal(s: &mut Located<&str>) -> PResult<Token> {
    take_while(1.., ('0'..='9', '_'))
        .verify(|s: &str| s.chars().any(|c| c != '_'))
        .with_span()
        .map(token(TokenKind::LiteralDecimal))
        .parse_next(s)
}

pub fn parse_hexadecimal(s: &mut Located<&str>) -> PResult<Token> {
    preceded(
        "0x",
        take_while(1.., ('0'..='9', 'a'..='f', 'A'..='F', '_')),
    )
    .verify(|s: &str| s.chars().any(|c| c != '_'))
    .with_span()
    .map(token(TokenKind::LiteralHexadecimal))
    .parse_next(s)
}

pub fn parse_binary(s: &mut Located<&str>) -> PResult<Token> {
    preceded("0b", take_while(1.., ('0'..='1', '_')))
        .verify(|s: &str| s.chars().any(|c| c != '_'))
        .with_span()
        .map(token(TokenKind::LiteralBinary))
        .parse_next(s)
}
