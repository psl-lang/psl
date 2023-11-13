use super::{Declaration, Expression, NameExpression};

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Statement {
    Declaration(Declaration),
    Write(WriteStatement),
    Expression(Expression),
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct WriteStatement {
    pub name: NameExpression,
}
