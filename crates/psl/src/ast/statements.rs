use super::{Declaration, Expression, NameExpression};

#[derive(Debug)]
pub enum Statement {
    Declaration(Declaration),
    Write(WriteStatement),
    Expression(Expression),
}

#[derive(Debug)]
pub struct WriteStatement {
    pub name: NameExpression,
}
