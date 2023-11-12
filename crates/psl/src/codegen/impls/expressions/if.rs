use crate::{
    ast::{ExpressionOrBlock, IfExpression},
    codegen::{construct::Type, context::CodegenContext, visitor::CodegenNode},
};

impl CodegenNode for IfExpression {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        let result_variable = ctx.generate_random_name();

        let ty = self.infer_type(ctx).unwrap();
        ctx.push_main(&ty.as_c_type());
        ctx.push_main(" ");
        ctx.push_main(&result_variable);
        ctx.push_main(";\n");

        let condition = ctx.visit(self.condition);
        ctx.push_main("if (");
        ctx.push_main(&condition);
        ctx.push_main(") {\n");

        let positive = match *self.positive {
            ExpressionOrBlock::Expression(expr) => ctx.visit(expr),
            ExpressionOrBlock::Block(block) => {
                for statement in block.statements {
                    ctx.visit(statement);
                }
                match block.last_expression {
                    Some(last_expr) => ctx.visit(last_expr),
                    None => "tuple0 {}".to_string(),
                }
            }
        };
        ctx.push_main(&result_variable);
        ctx.push_main(" = ");
        ctx.push_main(&positive);
        ctx.push_main(";\n");

        ctx.push_main("} else {\n");

        let negative = match self.negative.map(|x| *x) {
            Some(ExpressionOrBlock::Expression(expr)) => ctx.visit(expr),
            Some(ExpressionOrBlock::Block(block)) => {
                for statement in block.statements {
                    ctx.visit(statement);
                }
                match block.last_expression {
                    Some(last_expr) => ctx.visit(last_expr),
                    None => "tuple0 {}".to_string(),
                }
            }
            None => "tuple0 {}".to_string(),
        };
        ctx.push_main(&result_variable);
        ctx.push_main(" = ");
        ctx.push_main(&negative);
        ctx.push_main(";\n}\n");

        result_variable
    }
}

impl IfExpression {
    pub fn infer_type(&self, ctx: &CodegenContext) -> Result<Type, String> {
        let positive = match self.positive.as_ref() {
            ExpressionOrBlock::Expression(expr) => expr.infer_type(ctx)?,
            ExpressionOrBlock::Block(block) => block
                .last_expression
                .as_ref()
                .map(|expr| expr.infer_type(ctx))
                .transpose()?
                .unwrap_or(Type::UNIT),
        };
        let negative = match self.negative.as_deref() {
            Some(ExpressionOrBlock::Expression(expr)) => expr.infer_type(ctx)?,
            Some(ExpressionOrBlock::Block(block)) => block
                .last_expression
                .as_ref()
                .map(|expr| expr.infer_type(ctx))
                .transpose()?
                .unwrap_or(Type::UNIT),
            None => Type::UNIT,
        };

        positive.union_with(negative)
    }
}
