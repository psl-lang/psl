use winnow::{
    combinator::{cut_err, opt, terminated},
    error::{StrContext, StrContextValue},
    Located, PResult, Parser,
};

use crate::{
    ast::{TokenKind, WriteStatement},
    syntax::{expressions::parse_name, fragments::separator::parse_separator},
};

pub fn parse_write(s: &mut Located<&str>) -> PResult<WriteStatement> {
    (
        TokenKind::KeywordWrite,
        opt(TokenKind::WhitespaceHorizontal),
        cut_err(terminated(
            cut_err(parse_name).context(StrContext::Expected(StrContextValue::Description("name"))),
            parse_separator,
        )),
    )
        .map(|(_, _, name)| WriteStatement { name })
        .parse_next(s)
}
