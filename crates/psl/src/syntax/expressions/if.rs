use winnow::{
    combinator::{alt, opt, preceded},
    Located, PResult, Parser,
};

use crate::{
    ast::{ExpressionOrBlock, IfExpression, TokenKind},
    syntax::fragments::block::parse_block,
};

use super::parse_expression;

pub fn parse_if(s: &mut Located<&str>) -> PResult<IfExpression> {
    (
        TokenKind::KeywordIf,
        preceded(TokenKind::WhitespaceHorizontal, parse_expression).map(Box::new),
        preceded(TokenKind::WhitespaceHorizontal, parse_block)
            .map(|block| Box::new(ExpressionOrBlock::Block(block))),
        opt((
            preceded(TokenKind::WhitespaceHorizontal, TokenKind::KeywordElse),
            alt((preceded(TokenKind::WhitespaceHorizontal, parse_block)
                .map(|block| Box::new(ExpressionOrBlock::Block(block))),)),
        )),
    )
        .map(|(_, condition, positive, negative)| IfExpression {
            condition,
            positive,
            negative: negative.map(|(_, negative)| negative),
        })
        .parse_next(s)
}
