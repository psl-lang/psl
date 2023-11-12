mod write;

use crate::{
    ast::Statement,
    codegen::{scope::Scope, visitor::CodegenNode},
};

impl CodegenNode for Statement {
    fn produce_code(
        self,
        ctx: &mut crate::codegen::context::CodegenContext,
        scope: &mut Scope,
    ) -> String {
        match self {
            Statement::Declaration(node) => node.produce_code(ctx, scope),
            Statement::Write(node) => node.produce_code(ctx, scope),
            Statement::Expression(node) => {
                let output = node.produce_code(ctx, scope);
                ctx.push_main(&format!("{};", output));
                "".to_owned()
            }
        }
    }
}
