use crate::{
    ast::BlockExpression,
    codegen::{construct::Type, context::CodegenContext, visitor::CodegenNode},
};

impl CodegenNode for BlockExpression {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        let mut output = String::new();

        for statement in self.0.statements {
            output.push_str(&ctx.visit(statement));
        }

        if let Some(last_expression) = self.0.last_expression {
            output.push_str(&ctx.visit(last_expression));
        }

        output
    }
}

impl BlockExpression {
    pub fn infer_type(&self, ctx: &CodegenContext) -> Result<Type, String> {
        Ok(self
            .0
            .last_expression
            .as_ref()
            .map(|expr| expr.infer_type(ctx))
            .transpose()?
            .unwrap_or(Type::Tuple(Vec::new())))
    }
}
