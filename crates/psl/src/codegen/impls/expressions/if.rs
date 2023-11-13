use crate::{
    ast::IfExpression,
    codegen::{
        construct::{Scope, Type},
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
        output.push_str(&ctx.visit(self.negative));
        output.push(')');

        output
    }
}

impl NameResolutionPass for IfExpression {
    fn resolve(&self, _ctx: &mut NameResolutionContext) {}
}

impl IfExpression {
    pub fn infer_type(&self, scope: &Scope) -> Result<Type, String> {
        let positive = self.positive.infer_type(scope)?;
        let negative = self.negative.infer_type(scope)?;

        positive.union_with(negative)
    }
}
