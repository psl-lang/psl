use super::{Block, Declaration, Expression, FormatSpecifierFragment, Token};

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Statement {
    Declaration(Declaration),
    Write(WriteStatement),
    While(WhileStatement),
    CompoundAssignment(CompoundAssignmentStatement),
    Expression(Expression),
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct WriteStatement {
    pub fragments: Vec<FormatSpecifierFragment>,
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct WhileStatement {
    pub condition: Expression,
    pub block: Block,
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum CompoundAssignmentStatement {
    Simple(Assignment),
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Assignment {
    pub name: Token,
    pub expr: Expression,
}
