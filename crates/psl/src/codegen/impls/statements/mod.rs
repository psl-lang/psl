mod r#while;
mod write;

use crate::{
    ast::Statement,
    codegen::{
        pass::{NameResolutionContext, NameResolutionPass},
        visitor::CodegenNode,
    },
};

impl CodegenNode for Statement {
    fn produce_code(self, ctx: &mut crate::codegen::context::CodegenContext) -> String {
        match self {
            Statement::Declaration(node) => ctx.visit(node),
            Statement::Write(node) => ctx.visit(node),
            Statement::While(node) => ctx.visit(node),
            Statement::Expression(node) => format!("{};", ctx.visit(node)),
        }
    }
}

impl NameResolutionPass for Statement {
    fn resolve(&self, ctx: &mut NameResolutionContext) {
        match self {
            Statement::Declaration(node) => ctx.visit(node),
            Statement::Write(node) => ctx.visit(node),
            Statement::While(node) => ctx.visit(node),
            Statement::Expression(node) => ctx.visit(node),
        }
    }
}
