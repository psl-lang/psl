use winnow::{
    combinator::{alt, opt, repeat},
    Located, PResult, Parser,
};

use crate::{
    ast::TokenKind,
    syntax::tokens::{is_token, parse_token},
};

pub fn parse_separator(s: &mut Located<&str>) -> PResult<()> {
    alt((
        repeat::<_, _, Vec<_>, _, _>(
            1..,
            (
                opt(parse_token.verify(is_token(TokenKind::WhitespaceHorizontal))),
                parse_token.verify(is_token(TokenKind::WhitespaceVertical)),
            ),
        )
        .void(),
        parse_token.verify(is_token(TokenKind::Eof)).void(),
    ))
    .parse_next(s)
}
