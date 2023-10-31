use crate::{
    ast::IfExpression,
    codegen::{context::CodegenContext, visitor::CodegenNode},
};

impl CodegenNode for IfExpression {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        let mut output = String::new();

        output.push_str("((");
        output.push_str(&ctx.visit(self.condition));
        output.push_str(") ? (");
        output.push_str(&ctx.visit(self.positive));
        output.push_str(") : (");
        output.push_str(&ctx.visit(self.negative));
        output.push_str("))");
        
        output
    }
}
