use crate::ast::Program;

use self::{context::CodegenContext, scope::Scope, visitor::CodegenNode};

pub mod construct;
mod context;
mod impls;
mod scope;
mod visitor;

pub fn generate_codes(ast: Program) -> String {
    let mut ctx = CodegenContext::new();
    let mut scope = Scope::new();

    ast.produce_code(&mut ctx, &mut scope);

    ctx.output()
}
