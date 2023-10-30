mod binary_operator;
mod name;
mod read;

use crate::{
    ast::Expression,
    codegen::{context::CodegenContext, visitor::CodegenNode},
};

impl CodegenNode for Expression {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        match self {
            Expression::Read(node) => ctx.visit(node),
            Expression::Name(node) => ctx.visit(node),
            Expression::BinaryOperator(node) => ctx.visit(node),
        }
    }
}
