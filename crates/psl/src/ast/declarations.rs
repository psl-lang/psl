use super::{Expression, Token, Type};

#[derive(Debug)]
pub enum Declaration {
    Variable(VariableDeclaration),
}

#[derive(Debug, PartialEq)]
pub struct VariableDeclaration {
    pub ty: Type,
    pub name: Token,
    pub value: Option<Expression>,
}
