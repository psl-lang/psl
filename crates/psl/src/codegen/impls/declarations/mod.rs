mod variable_declaration;

use crate::{
    ast::Declaration,
    codegen::{context::CodegenContext, visitor::CodegenNode},
};

impl CodegenNode for Declaration {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        match self {
            Declaration::Variable(node) => ctx.visit(node),
        }
    }
}
