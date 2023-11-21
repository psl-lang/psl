use winnow::{
    combinator::{cut_err, opt},
    Located, PResult, Parser,
};

use crate::{
    ast::{TokenKind, WhileStatement},
    syntax::{
        expressions::parse_expression,
        fragments::{block::parse_block, separator::parse_separator},
    },
};

pub fn parse_while(s: &mut Located<&str>) -> PResult<WhileStatement> {
    (
        TokenKind::KeywordWhile,
        opt(TokenKind::WhitespaceHorizontal),
        cut_err(parse_expression),
        opt(TokenKind::WhitespaceHorizontal),
        cut_err(parse_block),
        parse_separator,
    )
        .map(|(_, _, condition, _, block, _)| WhileStatement { condition, block })
        .parse_next(s)
}
