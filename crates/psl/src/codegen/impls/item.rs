use crate::{
    ast::Item,
    codegen::{
        context::CodegenContext,
        pass::{NameResolutionContext, NameResolutionPass},
        visitor::CodegenNode,
    },
};

impl CodegenNode for Item {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        match self {
            Item::Declaration(node) => ctx.visit(node),
            Item::Statement(node) => ctx.visit(node),
        }
    }
}

impl NameResolutionPass for Item {
    fn resolve(&self, ctx: &mut NameResolutionContext) {
        match self {
            Item::Declaration(node) => ctx.visit(node),
            Item::Statement(node) => ctx.visit(node),
        }
    }
}
