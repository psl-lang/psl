use super::{Token, Type};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Read(ReadExpression),
    Name(NameExpression),
    BinaryOperator(BinaryOperatorExpression),
}

#[derive(Debug, PartialEq)]
pub enum ReadExpression {
    Type(Type),
}

#[derive(Debug, PartialEq)]
pub struct NameExpression {
    pub name: Token,
}

#[derive(Debug, PartialEq)]
pub struct BinaryOperatorExpression {
    pub lhs: Box<Expression>,
    pub operator: BinaryOperator,
    pub rhs: Box<Expression>,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}
