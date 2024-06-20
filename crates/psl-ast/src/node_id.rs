#[derive(Hash, PartialEq, Eq)]
pub struct NodeId(usize);

impl NodeId {
    const DUMMY: NodeId = NodeId(usize::MAX);

    pub fn is_dummy(&self) -> bool {
        self == &NodeId::DUMMY
    }
}
