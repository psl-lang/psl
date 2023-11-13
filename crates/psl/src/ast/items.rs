use super::{Declaration, Statement};

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Item {
    Declaration(Declaration),
    Statement(Statement),
}
