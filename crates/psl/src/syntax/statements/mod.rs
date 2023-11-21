mod r#while;
mod write;

use winnow::{
    combinator::{alt, opt, terminated},
    Located, PResult, Parser,
};

use crate::ast::Statement;

use self::{r#while::parse_while, write::parse_write};

use super::{
    declarations::parse_declaration, expressions::parse_expression,
    fragments::separator::parse_separator,
};

pub fn parse_statement(s: &mut Located<&str>) -> PResult<Statement> {
    alt((
        parse_declaration.map(Statement::Declaration),
        parse_write.map(Statement::Write),
        parse_while.map(Statement::While),
        terminated(parse_expression, opt(parse_separator)).map(Statement::Expression),
    ))
    .parse_next(s)
}
