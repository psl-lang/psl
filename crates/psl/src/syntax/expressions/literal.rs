use winnow::{combinator::alt, Located, PResult, Parser};

use crate::ast::{LiteralExpression, TokenKind};

pub fn parse_literal(s: &mut Located<&str>) -> PResult<LiteralExpression> {
    alt((
        TokenKind::LiteralDecimal,
        TokenKind::LiteralHexadecimal,
        TokenKind::LiteralBinary,
    ))
    .map(|token| LiteralExpression { value: token })
    .parse_next(s)
}
