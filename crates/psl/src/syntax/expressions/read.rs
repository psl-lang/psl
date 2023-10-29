use winnow::{
    combinator::{opt, preceded},
    Located, PResult, Parser,
};

use crate::{
    ast::{ReadExpression, TokenKind},
    syntax::fragments::r#type::parse_type,
};

pub fn parse_read(s: &mut Located<&str>) -> PResult<ReadExpression> {
    (
        TokenKind::KeywordRead,
        opt(preceded(TokenKind::WhitespaceHorizontal, parse_type)),
    )
        .map(|(_, ty)| ReadExpression { ty })
        .parse_next(s)
}
