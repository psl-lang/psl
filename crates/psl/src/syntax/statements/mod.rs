use winnow::{combinator::alt, Located, PResult, Parser};

use crate::ast::Statement;

use super::{declarations::parse_declaration, expressions::parse_expression};

pub fn parse_statement(s: &mut Located<&str>) -> PResult<Statement> {
    alt((
        parse_declaration.map(Statement::Declaration),
        parse_expression.map(Statement::Expression),
    ))
    .parse_next(s)
}
