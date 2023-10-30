use super::context::CodegenContext;

pub trait CodegenNode {
    fn produce_code(self, ctx: &mut CodegenContext) -> String;
}

impl CodegenContext {
    pub fn visit(&mut self, node: impl CodegenNode) -> String {
        node.produce_code(self)
    }
}
