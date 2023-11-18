use crate::{
    ast::{ExpressionOrBlock, IfExpression},
    codegen::{
        construct::Type,
        context::CodegenContext,
        pass::{NameResolutionContext, NameResolutionPass},
        visitor::CodegenNode,
    },
};

impl CodegenNode for IfExpression {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        impl CodegenNode for ExpressionOrBlock {
            fn produce_code(self, ctx: &mut CodegenContext) -> String {
                match self {
                    ExpressionOrBlock::Expression(expr) => ctx.visit(expr),
                    ExpressionOrBlock::Block(block) => {
                        let mut block_output = String::new();
                        block_output.push_str("{\n");
                        for statement in block.statements {
                            block_output.push_str(&ctx.visit(statement));
                        }
                        match block.last_expression {
                            Some(last_expr) => block_output.push_str(&ctx.visit(last_expr)),
                            None => block_output.push_str("tuple0 {}"),
                        }
                        block_output.push(';');
                        block_output.push_str("}\n");
                        block_output
                    }
                }
            }
        }
        let mut output = String::new();

        output.push('(');
        output.push_str(&ctx.visit(self.condition));
        output.push_str(") ? (");
        output.push_str(&ctx.visit(self.positive));
        output.push_str(") : (");
        match self.negative {
            Some(negative) => output.push_str(&ctx.visit(negative)),
            None => output.push_str("tuple0 {}"),
        };
        output.push(')');

        output
    }
}

impl NameResolutionPass for IfExpression {
    fn resolve(&self, ctx: &mut NameResolutionContext) {
        impl NameResolutionPass for ExpressionOrBlock {
            fn resolve(&self, ctx: &mut NameResolutionContext) {
                match self {
                    ExpressionOrBlock::Expression(node) => ctx.visit(node),
                    ExpressionOrBlock::Block(block) => {
                        for statement in &block.statements {
                            ctx.visit(statement);
                        }
                        ctx.visit(&block.last_expression)
                    }
                }
            }
        }
        ctx.visit(&self.condition);

        ctx.create_subscope().visit(&self.positive);
        ctx.create_subscope().visit(&self.negative);
    }
}

impl IfExpression {
    pub fn infer_type(&self, ctx: &CodegenContext) -> Result<Type, String> {
        impl ExpressionOrBlock {
            fn infer_type(&self, ctx: &CodegenContext) -> Result<Type, String> {
                match self {
                    ExpressionOrBlock::Expression(expr) => expr.infer_type(ctx),
                    ExpressionOrBlock::Block(block) => Ok(block
                        .last_expression
                        .as_ref()
                        .map(|expr| expr.infer_type(ctx))
                        .transpose()?
                        .unwrap_or(Type::UNIT)),
                }
            }
        }

        let positive = self.positive.infer_type(ctx)?;
        let negative = match self.negative.as_ref() {
            Some(negative) => negative.infer_type(ctx)?,
            None => Type::UNIT,
        };

        positive.union_with(negative)
    }
}
