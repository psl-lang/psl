use crate::{
    ast::Item,
    codegen::{context::CodegenContext, scope::Scope, visitor::CodegenNode},
};

impl CodegenNode for Item {
    fn produce_code(self, ctx: &mut CodegenContext, scope: &mut Scope) -> String {
        match self {
            Item::Declaration(node) => node.produce_code(ctx, scope),
            Item::Statement(node) => node.produce_code(ctx, scope),
        }
    }
}
