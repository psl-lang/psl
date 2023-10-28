use winnow::{combinator::alt, token::tag, Located, PResult, Parser};

use crate::ast::{Token, TokenKind};

use super::token;

pub fn parse_punctuations(s: &mut Located<&str>) -> PResult<Token> {
    alt([
        tag("+")
            .with_span()
            .map(token(TokenKind::PunctuationPlusSign)),
        tag("=")
            .with_span()
            .map(token(TokenKind::PonctuationEqualsSign)),
    ])
    .parse_next(s)
}
