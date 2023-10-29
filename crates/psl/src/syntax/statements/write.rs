use winnow::{combinator::opt, Located, PResult, Parser};

use crate::{
    ast::{TokenKind, WriteStatement},
    syntax::expressions::parse_name,
};

pub fn parse_write(s: &mut Located<&str>) -> PResult<WriteStatement> {
    (
        TokenKind::KeywordWrite,
        opt(TokenKind::WhitespaceHorizontal),
        parse_name,
    )
        .map(|(_, _, name)| WriteStatement { name })
        .parse_next(s)
}
