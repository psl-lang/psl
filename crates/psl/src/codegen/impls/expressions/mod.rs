mod binary_operator;
mod r#if;
mod literal;
mod name;
mod read;

use crate::{
    ast::Expression,
    codegen::{
        construct::Type,
        context::CodegenContext,
        pass::{NameResolutionContext, NameResolutionPass},
        visitor::CodegenNode,
    },
};

impl CodegenNode for Expression {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        match self {
            Expression::Literal(node) => ctx.visit(node),
            Expression::Read(node) => ctx.visit(node),
            Expression::Name(node) => ctx.visit(node),
            Expression::If(node) => ctx.visit(node),
            Expression::BinaryOperator(node) => ctx.visit(node),
        }
    }
}

impl NameResolutionPass for Expression {
    fn resolve(&self, ctx: &mut NameResolutionContext) {
        match self {
            Expression::Literal(node) => ctx.visit(node),
            Expression::Read(node) => ctx.visit(node),
            Expression::Name(node) => ctx.visit(node),
            Expression::If(node) => ctx.visit(node),
            Expression::BinaryOperator(node) => ctx.visit(node),
        }
    }
}

impl Expression {
    pub fn infer_type(&self, ctx: &CodegenContext) -> Result<Type, String> {
        match self {
            Expression::Literal(expr) => expr.infer_type(ctx),
            Expression::Read(expr) => expr.infer_type(ctx),
            Expression::Name(expr) => expr.infer_type(ctx),
            Expression::If(expr) => expr.infer_type(ctx),
            Expression::BinaryOperator(expr) => expr.infer_type(ctx),
        }
    }
}
