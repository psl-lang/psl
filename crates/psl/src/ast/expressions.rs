use super::{Block, Token, Type};

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Expression {
    Read(ReadExpression),
    Name(NameExpression),
    If(IfExpression),
    BinaryOperator(BinaryOperatorExpression),
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum ReadExpression {
    Type(Type),
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct NameExpression {
    pub name: Token,
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct IfExpression {
    pub condition: Box<Expression>,
    pub positive: Box<ExpressionOrBlock>,
    pub negative: Option<Box<ExpressionOrBlock>>,
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum ExpressionOrBlock {
    Expression(Expression),
    Block(Block),
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct BinaryOperatorExpression {
    pub lhs: Box<Expression>,
    pub operator: BinaryOperator,
    pub rhs: Box<Expression>,
}

#[derive(Clone, Debug, Hash, PartialEq)]
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
