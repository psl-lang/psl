use std::{fmt::Display, ops::Range};

use winnow::{
    combinator::{alt, success},
    error::ContextError,
    token::any,
    Located, PResult, Parser,
};

use crate::ast::{Token, TokenKind};

use self::{
    identifier::parse_identifier_identifier,
    keyword::parse_keyword,
    literal::parse_literal,
    punctuations::parse_punctuations,
    whitespaces::{parse_whitespace_horizontal, parse_whitespace_vertical},
};

mod identifier;
mod keyword;
mod literal;
mod punctuations;
mod whitespaces;

pub fn parse_token(s: &mut Located<&str>) -> PResult<Token> {
    alt((
        parse_keyword,
        parse_punctuations,
        parse_whitespace_horizontal,
        parse_whitespace_vertical,
        parse_identifier_identifier,
        parse_literal,
        any.with_span().map(token(TokenKind::Error)),
        success("<eof>").with_span().map(token(TokenKind::Eof)),
    ))
    .parse_next(s)
}

#[inline(always)]
pub(super) fn token<T: Display>(kind: TokenKind) -> impl Fn((T, Range<usize>)) -> Token {
    move |(content, span)| Token {
        kind: kind.clone(),
        content: content.to_string(),
        span,
    }
}

impl Parser<Located<&str>, Token, ContextError> for TokenKind {
    fn parse_next(&mut self, s: &mut Located<&str>) -> PResult<Token, ContextError> {
        parse_token
            .verify(|token| token.kind == self.clone())
            .parse_next(s)
    }
}
