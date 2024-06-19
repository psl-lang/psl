use crate::{
    ast::IfExpression,
    codegen::{
        construct::Type,
        context::CodegenContext,
        pass::{NameResolutionContext, NameResolutionPass},
        visitor::CodegenNode,
    },
};

impl CodegenNode for IfExpression {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        let mut output = String::new();

        output.push('(');
        output.push_str(&ctx.visit(self.condition));
        output.push_str(") ? (");
        output.push_str(&ctx.visit(self.positive));
        output.push_str(") : (");
        match self.negative {
            Some(negative) => output.push_str(&ctx.visit(negative)),
            None => output.push_str("{}"),
        };
        output.push(')');

        output
    }
}

impl NameResolutionPass for IfExpression {
    fn resolve(&self, ctx: &mut NameResolutionContext) {
        ctx.visit(&self.condition);

        ctx.create_subscope().visit(&self.positive);
        ctx.create_subscope().visit(&self.negative);
    }
}

impl IfExpression {
    pub fn infer_type(&self, ctx: &CodegenContext) -> Result<Type, String> {
        let positive = self.positive.infer_type(ctx)?;
        let negative = self
            .negative
            .as_ref()
            .map(|negative| negative.infer_type(ctx))
            .transpose()?
            .unwrap_or(Type::UNIT);

        positive.union_with(negative)
    }
}
