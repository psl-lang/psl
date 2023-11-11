use super::{Expression, Token, Type};

#[derive(Clone, Debug, PartialEq)]
pub enum Declaration {
    Variable(VariableDeclaration),
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariableDeclaration {
    pub ty: Type,
    pub name: Token,
    pub value: Option<Expression>,
}
