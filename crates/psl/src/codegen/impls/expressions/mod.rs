mod binary_operator;
mod r#if;
mod name;
mod read;

use crate::{
    ast::Expression,
    codegen::{construct::Type, context::CodegenContext, scope::Scope, visitor::CodegenNode},
};

impl CodegenNode for Expression {
    fn produce_code(self, ctx: &mut CodegenContext, scope: &mut Scope) -> String {
        match self {
            Expression::Read(node) => node.produce_code(ctx, scope),
            Expression::Name(node) => node.produce_code(ctx, scope),
            Expression::If(node) => node.produce_code(ctx, scope),
            Expression::BinaryOperator(node) => node.produce_code(ctx, scope),
        }
    }
}

impl Expression {
    pub fn infer_type(&self, ctx: &CodegenContext, scope: &mut Scope) -> Result<Type, String> {
        match self {
            Expression::Read(expr) => expr.infer_type(ctx, scope),
            Expression::Name(expr) => expr.infer_type(ctx, scope),
            Expression::If(expr) => expr.infer_type(ctx, scope),
            Expression::BinaryOperator(expr) => expr.infer_type(ctx, scope),
        }
    }
}
