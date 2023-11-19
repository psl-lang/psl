use crate::{
    ast::ExpressionOrBlock,
    codegen::{
        construct::Type,
        context::CodegenContext,
        pass::{NameResolutionContext, NameResolutionPass},
        visitor::CodegenNode,
    },
};

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
                block_output.push_str(";\n}");
                block_output
            }
        }
    }
}

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

impl ExpressionOrBlock {
    pub fn infer_type(&self, ctx: &CodegenContext) -> Result<Type, String> {
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
