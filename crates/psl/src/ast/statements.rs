use super::{Block, Declaration, Expression, NameExpression};

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Statement {
    Declaration(Declaration),
    Write(WriteStatement),
    While(WhileStatement),
    Expression(Expression),
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct WriteStatement {
    pub name: NameExpression,
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct WhileStatement {
    pub condition: Expression,
    pub block: Block,
}
