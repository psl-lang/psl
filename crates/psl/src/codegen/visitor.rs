use super::context::CodegenContext;

pub trait CodegenNode {
    fn produce_code(self, ctx: &mut CodegenContext) -> String;
}

impl<T: CodegenNode> CodegenNode for Box<T> {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        T::produce_code(*self, ctx)
    }
}

impl CodegenContext {
    pub fn visit(&mut self, node: impl CodegenNode) -> String {
        node.produce_code(self)
    }
}
