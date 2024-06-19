use function_declaration::parse_function_declaration;
use winnow::{combinator::alt, Located, PResult, Parser};

use crate::ast::Declaration;

use self::variable_declaration::parse_variable_declaration;

mod function_declaration;
mod variable_declaration;

pub fn parse_declaration(s: &mut Located<&str>) -> PResult<Declaration> {
    alt((
        parse_variable_declaration.map(Declaration::Variable),
        parse_function_declaration.map(Declaration::Function),
    ))
    .parse_next(s)
}
