use winnow::{combinator::alt, Located, PResult, Parser};

use crate::ast::{Token, TokenKind};

use super::token;

pub fn parse_keyword(s: &mut Located<&str>) -> PResult<Token> {
    // please sort alt by enum declaration order
    alt((
        kw("else", TokenKind::KeywordElse),
        kw("fn", TokenKind::KeywordFn),
        kw("if", TokenKind::KeywordIf),
        kw("read", TokenKind::KeywordRead),
        kw("return", TokenKind::KeywordReturn),
        kw("write", TokenKind::KeywordWrite),
        kw("while", TokenKind::KeywordWhile),
    ))
    .parse_next(s)
}

fn kw(
    keyword: &str,
    kind: TokenKind,
) -> impl for<'a> Fn(&mut Located<&'a str>) -> PResult<Token> + '_ {
    move |s| keyword.with_span().map(token(kind.clone())).parse_next(s)
}
