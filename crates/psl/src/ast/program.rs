use super::Item;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Program {
    pub items: Vec<Item>,
}
