use winnow::{combinator::alt, token::tag, Located, PResult, Parser};

use crate::ast::{Token, TokenKind};

use super::token;

pub fn parse_keyword(s: &mut Located<&str>) -> PResult<Token> {
    // please sort alt by enum declaration order
    alt((
        tag("else").with_span().map(token(TokenKind::KeywordElse)),
        tag("fn").with_span().map(token(TokenKind::KeywordFn)),
        tag("if").with_span().map(token(TokenKind::KeywordIf)),
        tag("read").with_span().map(token(TokenKind::KeywordRead)),
        tag("return")
            .with_span()
            .map(token(TokenKind::KeywordReturn)),
        tag("write").with_span().map(token(TokenKind::KeywordWrite)),
        tag("while").with_span().map(token(TokenKind::KeywordWhile)),
    ))
    .parse_next(s)
}
