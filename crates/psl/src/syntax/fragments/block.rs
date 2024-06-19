use winnow::{
    combinator::{cut_err, opt, repeat},
    Located, PResult, Parser,
};

use crate::{
    ast::{Block, Statement, TokenKind},
    syntax::{fragments::separator::parse_separator, statements::parse_statement},
};

pub fn parse_block(s: &mut Located<&str>) -> PResult<Block> {
    (
        TokenKind::PunctuationLeftCurlyBracket,
        cut_err((
            opt(TokenKind::WhitespaceHorizontal),
            opt(parse_separator),
            repeat::<_, _, Vec<_>, _, _>(0.., parse_statement),
            opt(parse_separator),
            opt(TokenKind::WhitespaceHorizontal),
            TokenKind::PunctuationRightCurlyBracket,
        )),
    )
        .map(|(_, (_, _, statements, _, _, _))| match &statements[..] {
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
