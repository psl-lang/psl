mod variable_declaration;

use crate::{
    ast::Declaration,
    codegen::{context::CodegenContext, scope::Scope, visitor::CodegenNode},
};

impl CodegenNode for Declaration {
    fn produce_code(self, ctx: &mut CodegenContext, scope: &mut Scope) -> String {
        match self {
            Declaration::Variable(node) => node.produce_code(ctx, scope),
        }
    }
}
