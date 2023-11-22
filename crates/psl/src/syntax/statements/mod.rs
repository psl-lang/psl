mod assignment;
mod r#while;
mod write;

use winnow::{
    combinator::{alt, opt, terminated},
    Located, PResult, Parser,
};

use crate::ast::Statement;

use self::{assignment::parse_compound_assignment, r#while::parse_while, write::parse_write};

use super::{
    declarations::parse_declaration, expressions::parse_expression,
    fragments::separator::parse_separator,
};

pub fn parse_statement(s: &mut Located<&str>) -> PResult<Statement> {
    alt((
        parse_declaration.map(Statement::Declaration),
        parse_write.map(Statement::Write),
        parse_while.map(Statement::While),
        parse_compound_assignment.map(Statement::CompoundAssignment),
        terminated(parse_expression, opt(parse_separator)).map(Statement::Expression),
    ))
    .parse_next(s)
}
