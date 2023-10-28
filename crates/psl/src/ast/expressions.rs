use super::Type;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Read(ReadExpression),
}

#[derive(Debug, PartialEq)]
pub struct ReadExpression {
    pub ty: Option<Type>,
}
