use winnow::{
    combinator::{alt, opt, repeat},
    Located, PResult, Parser,
};

use crate::ast::TokenKind;

pub fn parse_separator(s: &mut Located<&str>) -> PResult<()> {
    alt((
        (
            repeat::<_, _, (), _, _>(
                1..,
                (
                    opt(TokenKind::WhitespaceHorizontal),
                    TokenKind::WhitespaceVertical,
                ),
            ),
            opt(TokenKind::WhitespaceHorizontal),
        )
            .void(),
        TokenKind::Eof.void(),
    ))
    .parse_next(s)
}
