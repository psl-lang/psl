use crate::ast::Program;

use self::{context::CodegenContext, pass::NameResolutionContext};

pub mod construct;
mod context;
mod impls;
mod pass;
mod visitor;

pub fn generate_codes(ast: Program) -> String {
    let mut resolution_ctx = NameResolutionContext::new();
    resolution_ctx.visit(&ast);

    let mut ctx = CodegenContext::new(resolution_ctx.finish());
    ctx.visit(ast)
}
