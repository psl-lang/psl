use winnow::{combinator::alt, token::tag, Located, PResult, Parser};

use crate::ast::{Token, TokenKind};

use super::token;

pub fn parse_keyword(s: &mut Located<&str>) -> PResult<Token> {
    alt((
        tag("read").with_span().map(token(TokenKind::KeywordRead)),
        tag("write").with_span().map(token(TokenKind::KeywordWrite)),
    ))
    .parse_next(s)
}
