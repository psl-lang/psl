use winnow::{combinator::preceded, Located, PResult, Parser};

use crate::ast::{ReturnExpression, TokenKind};

use super::parse_expression;

pub fn parse_return(s: &mut Located<&str>) -> PResult<ReturnExpression> {
    (
        TokenKind::KeywordReturn,
        preceded(TokenKind::WhitespaceHorizontal, parse_expression),
    )
        .map(|(_, value)| ReturnExpression {
            value: Box::new(value),
        })
        .parse_next(s)
}
