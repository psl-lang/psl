use crate::{
    ast::ReturnExpression,
    codegen::{
        construct::Type,
        context::CodegenContext,
        pass::{NameResolutionContext, NameResolutionPass},
        visitor::CodegenNode,
    },
};

impl CodegenNode for ReturnExpression {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        if self.value.infer_type(ctx).unwrap() == Type::Never {
            ctx.visit(self.value)
        } else {
            format!("return {};", ctx.visit(self.value))
        }
    }
}

impl NameResolutionPass for ReturnExpression {
    fn resolve(&self, ctx: &mut NameResolutionContext) {
        ctx.visit(&self.value);
    }
}

impl ReturnExpression {
    pub fn infer_type(&self, _: &CodegenContext) -> Result<Type, String> {
        Ok(Type::Never)
    }
}
