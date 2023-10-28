use std::{fmt::Display, ops::Range};

use winnow::{
    combinator::{alt, success},
    Located, PResult, Parser,
};

use crate::ast::{Token, TokenKind};

use self::{
    identifier::parse_identifier_identifier,
    keyword::parse_keyword,
    punctuations::parse_punctuations,
    whitespaces::{parse_whitespace_horizontal, parse_whitespace_vertical},
};

mod identifier;
mod keyword;
mod punctuations;
mod whitespaces;

pub fn parse_token(s: &mut Located<&str>) -> PResult<Token> {
    alt((
        parse_keyword,
        parse_punctuations,
        parse_whitespace_horizontal,
        parse_whitespace_vertical,
        parse_identifier_identifier,
        success("").with_span().map(token(TokenKind::Eof)),
    ))
    .parse_next(s)
}

pub fn is_token(kind: TokenKind) -> impl Fn(&Token) -> bool {
    move |token| token.kind == kind
}

#[inline(always)]
pub(super) fn token<T: Display>(kind: TokenKind) -> impl Fn((T, Range<usize>)) -> Token {
    move |(content, span)| Token {
        kind: kind.clone(),
        content: content.to_string(),
        span,
    }
}
