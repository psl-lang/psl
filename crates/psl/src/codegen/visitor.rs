use super::{context::CodegenContext, scope::Scope};

pub trait CodegenNode {
    fn produce_code(self, ctx: &mut CodegenContext, scope: &mut Scope) -> String;
}

impl<T: CodegenNode> CodegenNode for Box<T> {
    fn produce_code(self, ctx: &mut CodegenContext, scope: &mut Scope) -> String {
        T::produce_code(*self, ctx, scope)
    }
}
