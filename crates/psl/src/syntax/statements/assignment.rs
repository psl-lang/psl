use winnow::{
    combinator::{alt, opt},
    Located, PResult, Parser,
};

use crate::{
    ast::{Assignment, CompoundAssignmentStatement, TokenKind},
    syntax::{expressions::parse_expression, fragments::separator::parse_separator},
};

pub fn parse_compound_assignment(s: &mut Located<&str>) -> PResult<CompoundAssignmentStatement> {
    (
        TokenKind::IdentifierIdentifier,
        opt(TokenKind::WhitespaceHorizontal),
        alt(((TokenKind::PunctuationEqualsSign).map(|_| CompoundAssignmentStatement::Simple),)),
        opt(TokenKind::WhitespaceHorizontal),
        parse_expression,
        parse_separator,
    )
        .map(|(name, _, f, _, expr, _)| f(Assignment { name, expr }))
        .parse_next(s)
}
