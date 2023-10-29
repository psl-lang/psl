use winnow::{Located, PResult, Parser};

use crate::ast::{NameExpression, TokenKind};

pub fn parse_name(s: &mut Located<&str>) -> PResult<NameExpression> {
    TokenKind::IdentifierIdentifier
        .map(|name| NameExpression { name })
        .parse_next(s)
}
