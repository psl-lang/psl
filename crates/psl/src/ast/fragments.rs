use super::{Expression, Statement, Token};

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Type {
    Simple(Token),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub last_expression: Option<Box<Expression>>,
}
