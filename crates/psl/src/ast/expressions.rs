use super::{ExpressionOrBlock, Token, Type};

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Expression {
    Literal(LiteralExpression),
    Read(ReadExpression),
    Return(ReturnExpression),
    Name(NameExpression),
    If(IfExpression),
    BinaryOperator(BinaryOperatorExpression),
    PostfixOperator(PostfixOperatorExpression),
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct LiteralExpression {
    pub value: Token,
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum ReadExpression {
    Type(Type),
}
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct ReturnExpression {
    pub value: Box<Expression>,
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

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct PostfixOperatorExpression {
    pub expr: Box<Expression>,
    pub operator: PostfixOperator,
}
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum PostfixOperator {
    Invoke(Vec<Expression>),
    Index(Vec<Expression>),
}
