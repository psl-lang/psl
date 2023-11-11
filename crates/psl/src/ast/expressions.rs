use super::{Block, Token, Type};

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Read(ReadExpression),
    Name(NameExpression),
    If(IfExpression),
    BinaryOperator(BinaryOperatorExpression),
    Block(BlockExpression),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ReadExpression {
    Type(Type),
}

#[derive(Clone, Debug, PartialEq)]
pub struct NameExpression {
    pub name: Token,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfExpression {
    pub condition: Box<Expression>,
    pub positive: Box<Expression>,
    pub negative: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryOperatorExpression {
    pub lhs: Box<Expression>,
    pub operator: BinaryOperator,
    pub rhs: Box<Expression>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    LogiacalAnd,
    LogicalOr,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockExpression(pub Block);
