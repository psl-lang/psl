use winnow::{combinator::preceded, Located, PResult, Parser};

use crate::ast::{IfExpression, TokenKind};

use super::parse_expression;

pub fn parse_if(s: &mut Located<&str>) -> PResult<IfExpression> {
    (
        TokenKind::KeywordIf,
        preceded(TokenKind::WhitespaceHorizontal, parse_expression),
        preceded(TokenKind::WhitespaceHorizontal, TokenKind::KeywordThen),
        preceded(TokenKind::WhitespaceHorizontal, parse_expression),
        preceded(TokenKind::WhitespaceHorizontal, TokenKind::KeywordElse),
        preceded(TokenKind::WhitespaceHorizontal, parse_expression),
    )
        .map(|(_, condition, _, positive, _, negative)| IfExpression {
            condition: condition.into(),
            positive: positive.into(),
            negative: negative.into(),
        })
        .parse_next(s)
}
