use winnow::{
    combinator::{alt, opt, repeat},
    Located, PResult, Parser,
};

use crate::ast::TokenKind;

pub fn parse_separator(s: &mut Located<&str>) -> PResult<()> {
    alt((
        repeat::<_, _, Vec<_>, _, _>(
            1..,
            (
                opt(TokenKind::WhitespaceHorizontal),
                TokenKind::WhitespaceVertical,
            ),
        )
        .void(),
        TokenKind::Eof.void(),
    ))
    .parse_next(s)
}
