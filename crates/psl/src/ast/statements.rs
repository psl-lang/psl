use super::{Declaration, Expression, NameExpression};

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Declaration(Declaration),
    Write(WriteStatement),
    Expression(Expression),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WriteStatement {
    pub name: NameExpression,
}
