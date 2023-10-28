use super::{Declaration, Statement};

#[derive(Debug)]
pub enum Item {
    Declaration(Declaration),
    Statement(Statement),
}
