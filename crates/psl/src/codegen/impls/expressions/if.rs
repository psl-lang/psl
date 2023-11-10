use crate::{
    ast::IfExpression,
    codegen::{construct::Type, context::CodegenContext, visitor::CodegenNode},
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

impl IfExpression {
    pub fn infer_type(&self, ctx: &CodegenContext) -> Result<Type, String> {
        let positive = self.positive.infer_type(ctx)?;
        let negative = self.negative.infer_type(ctx)?;

        positive.union_with(negative)
    }
}
