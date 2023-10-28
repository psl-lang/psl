use winnow::{
    combinator::{opt, preceded},
    Located, PResult, Parser,
};

use crate::{
    ast::{ReadExpression, TokenKind},
    syntax::{
        fragments::r#type::parse_type,
        tokens::{is_token, parse_token},
    },
};

pub fn parse_read(s: &mut Located<&str>) -> PResult<ReadExpression> {
    (
        parse_token.verify(is_token(TokenKind::KeywordRead)),
        opt(preceded(
            parse_token.verify(is_token(TokenKind::WhitespaceHorizontal)),
            parse_type,
        )),
    )
        .map(|(_, ty)| ReadExpression { ty })
        .parse_next(s)
}
