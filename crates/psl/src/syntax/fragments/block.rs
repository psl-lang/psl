use winnow::{
    combinator::{delimited, opt, repeat},
    Located, PResult, Parser,
};

use crate::{
    ast::{Block, Statement, TokenKind},
    syntax::{fragments::separator::parse_separator, statements::parse_statement},
};

pub fn parse_block(s: &mut Located<&str>) -> PResult<Block> {
    delimited(
        TokenKind::PunctuationLeftCurlyBracket,
        delimited(
            opt(parse_separator),
            repeat(0.., parse_statement),
            opt(parse_separator),
        ),
        TokenKind::PunctuationRightCurlyBracket,
    )
    .map(|statements: Vec<_>| match &statements[..] {
        [statements @ .., Statement::Expression(expr)] => Block {
            statements: statements.to_vec(),
            last_expression: Some(Box::new(expr.clone())),
        },
        statements => Block {
            statements: statements.to_vec(),
            last_expression: None,
        },
    })
    .parse_next(s)
}
