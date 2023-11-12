use crate::{
    ast::{ExpressionOrBlock, IfExpression},
    codegen::{construct::Type, context::CodegenContext, scope::Scope, visitor::CodegenNode},
};

impl CodegenNode for IfExpression {
    fn produce_code(self, ctx: &mut CodegenContext, scope: &mut Scope) -> String {
        let result_variable = ctx.generate_random_name();

        let ty = self.infer_type(ctx, scope).unwrap();
        ctx.push_main(&ty.as_c_type());
        ctx.push_main(" ");
        ctx.push_main(&result_variable);
        ctx.push_main(";\n");

        let condition = self.condition.produce_code(ctx, scope);
        ctx.push_main("if (");
        ctx.push_main(&condition);
        ctx.push_main(") {\n");

        let positive = match *self.positive {
            ExpressionOrBlock::Expression(expr) => expr.produce_code(ctx, scope),
            ExpressionOrBlock::Block(block) => {
                let mut subscope = scope.create_subscope();

                for statement in block.statements {
                    statement.produce_code(ctx, &mut subscope);
                }
                dbg!(&subscope);
                match block.last_expression {
                    Some(last_expr) => last_expr.produce_code(ctx, &mut subscope),
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
            Some(ExpressionOrBlock::Expression(expr)) => expr.produce_code(ctx, scope),
            Some(ExpressionOrBlock::Block(block)) => {
                for statement in block.statements {
                    statement.produce_code(ctx, scope);
                }
                match block.last_expression {
                    Some(last_expr) => last_expr.produce_code(ctx, scope),
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
    pub fn infer_type(&self, ctx: &CodegenContext, scope: &mut Scope) -> Result<Type, String> {
        let positive = match self.positive.as_ref() {
            ExpressionOrBlock::Expression(expr) => expr.infer_type(ctx, scope)?,
            ExpressionOrBlock::Block(block) => block
                .last_expression
                .as_ref()
                .map(|expr| expr.infer_type(ctx, scope))
                .transpose()?
                .unwrap_or(Type::UNIT),
        };
        let negative = match self.negative.as_deref() {
            Some(ExpressionOrBlock::Expression(expr)) => expr.infer_type(ctx, scope)?,
            Some(ExpressionOrBlock::Block(block)) => block
                .last_expression
                .as_ref()
                .map(|expr| expr.infer_type(ctx, scope))
                .transpose()?
                .unwrap_or(Type::UNIT),
            None => Type::UNIT,
        };

        positive.union_with(negative)
    }
}
