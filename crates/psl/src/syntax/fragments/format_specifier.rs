use winnow::{Located, PResult, Parser};

use crate::{
    ast::{FormatSpecifier, TokenKind},
    syntax::parse_token,
};

pub fn parse_format_specifier(s: &mut Located<&str>) -> PResult<FormatSpecifier> {
    parse_token
        .verify_map(|t| {
            if let TokenKind::LiteralFormatSpecifier(s) = t.kind {
                Some(s)
            } else {
                None
            }
        })
        .parse_next(s)
}
