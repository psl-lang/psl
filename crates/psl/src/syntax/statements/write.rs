use winnow::{
    combinator::{alt, cut_err, opt, terminated},
    error::{StrContext, StrContextValue},
    Located, PResult, Parser,
};

use crate::{
    ast::{FormatSpecifierFragment, TokenKind, WriteStatement},
    syntax::{
        expressions::parse_name,
        fragments::{format_specifier::parse_format_specifier, separator::parse_separator},
    },
};

pub fn parse_write(s: &mut Located<&str>) -> PResult<WriteStatement> {
    (
        TokenKind::KeywordWrite,
        opt(TokenKind::WhitespaceHorizontal),
        cut_err(terminated(
            alt((
                parse_name.map(|name| WriteStatement {
                    fragments: vec![FormatSpecifierFragment::Variable(name.name.content)],
                }),
                parse_format_specifier.map(|format_specifier| WriteStatement {
                    fragments: format_specifier.0,
                }),
            )),
            parse_separator,
        ))
        .context(StrContext::Expected(StrContextValue::Description(
            "name or format specifier",
        ))),
    )
        .map(|(_, _, stmt)| stmt)
        .parse_next(s)
}
