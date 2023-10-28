use super::{Declaration, Expression};

#[derive(Debug)]
pub enum Statement {
    Declaration(Declaration),
    Expression(Expression),
}
