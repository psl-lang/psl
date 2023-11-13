mod binary_operator;
mod r#if;
mod name;
mod read;

use crate::{
    ast::Expression,
    codegen::{
        construct::{Scope, Type},
        context::CodegenContext,
        pass::{NameResolutionContext, NameResolutionPass},
        visitor::CodegenNode,
    },
};

impl CodegenNode for Expression {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        match self {
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
            Expression::Read(node) => ctx.visit(node),
            Expression::Name(node) => ctx.visit(node),
            Expression::If(node) => ctx.visit(node),
            Expression::BinaryOperator(node) => ctx.visit(node),
        }
    }
}

impl Expression {
    pub fn infer_type(&self, scope: &Scope) -> Result<Type, String> {
        match self {
            Expression::Read(expr) => expr.infer_type(scope),
            Expression::Name(expr) => expr.infer_type(scope),
            Expression::If(expr) => expr.infer_type(scope),
            Expression::BinaryOperator(_) => todo!(),
        }
    }
}
