use crate::{
    ast::Item,
    codegen::{context::CodegenContext, visitor::CodegenNode},
};

impl CodegenNode for Item {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        match self {
            Item::Declaration(node) => ctx.visit(node),
            Item::Statement(node) => ctx.visit(node),
        }
    }
}
