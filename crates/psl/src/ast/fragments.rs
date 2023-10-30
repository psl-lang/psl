use super::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Simple(Token),
}
