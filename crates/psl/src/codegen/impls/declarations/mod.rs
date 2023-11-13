mod variable_declaration;

use crate::{
    ast::Declaration,
    codegen::{
        context::CodegenContext,
        pass::{NameResolutionContext, NameResolutionPass},
        visitor::CodegenNode,
    },
};

impl CodegenNode for Declaration {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        match self {
            Declaration::Variable(node) => ctx.visit(node),
        }
    }
}

impl NameResolutionPass for Declaration {
    fn resolve(&self, ctx: &mut NameResolutionContext) {
        match self {
            Declaration::Variable(node) => ctx.visit(node),
        }
    }
}
