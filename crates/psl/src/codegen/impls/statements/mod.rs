mod write;

use crate::{ast::Statement, codegen::visitor::CodegenNode};

impl CodegenNode for Statement {
    fn produce_code(self, ctx: &mut crate::codegen::context::CodegenContext) -> String {
        match self {
            Statement::Declaration(node) => ctx.visit(node),
            Statement::Write(node) => ctx.visit(node),
            Statement::Expression(node) => {
                let output = ctx.visit(node);
                ctx.push_main(&format!("{};", output));
                "".to_owned()
            }
        }
    }
}
