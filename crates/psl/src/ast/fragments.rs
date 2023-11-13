use super::Token;

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Type {
    Simple(Token),
}
