use crate::{
    ast::{BinaryOperator, BinaryOperatorExpression},
    codegen::{construct::Type, context::CodegenContext, scope::Scope, visitor::CodegenNode},
};

impl CodegenNode for BinaryOperatorExpression {
    fn produce_code(self, ctx: &mut CodegenContext, scope: &mut Scope) -> String {
        if matches!(
            self.operator,
            BinaryOperator::LogiacalAnd | BinaryOperator::LogicalOr
        ) {
            let mut lhs = String::new();
            let output_name = ctx.generate_random_name();
            let ty = self.lhs.infer_type(ctx, scope).unwrap().as_c_type();
            lhs.push_str(&format!(
                "{} {} = {};\n",
                ty,
                &output_name,
                self.lhs.produce_code(ctx, scope)
            ));

            let should_negate = self.operator == BinaryOperator::LogicalOr;

            ctx.push_main(&lhs);

            ctx.push_main(&format!(
                "if ({}{}) {{\n",
                if should_negate { "!" } else { "" },
                &output_name,
            ));

            let rhs = self.rhs.produce_code(ctx, scope);
            ctx.push_main(&format!("{} = {};\n", &output_name, rhs));

            ctx.push_main("}\n");

            return output_name;
        }

        let mut output = String::new();
        output.push('(');
        output.push_str(&self.lhs.produce_code(ctx, scope));
        output.push(')');

        /*
         * @TODO:
         * when we introduce operator overloading, we should change this.
         */
        let operator = match self.operator {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Modulus => "%",
            BinaryOperator::Equal => "==",
            BinaryOperator::NotEqual => "!=",
            BinaryOperator::Less => "<",
            BinaryOperator::Greater => ">",
            BinaryOperator::LessEqual => "<=",
            BinaryOperator::GreaterEqual => "<=",
            BinaryOperator::LogiacalAnd | BinaryOperator::LogicalOr => unreachable!(),
        };

        output.push_str(operator);

        output.push('(');
        output.push_str(&self.rhs.produce_code(ctx, scope));
        output.push(')');

        output
    }
}

impl BinaryOperatorExpression {
    /**
     * @TODO:
     * when we introduce operator overloading, we should change this.
     */
    pub fn infer_type(&self, ctx: &CodegenContext, scope: &mut Scope) -> Result<Type, String> {
        match self.operator {
            BinaryOperator::Add
            | BinaryOperator::Subtract
            | BinaryOperator::Multiply
            | BinaryOperator::Divide
            | BinaryOperator::Modulus => self.lhs.infer_type(ctx, scope),

            BinaryOperator::Equal
            | BinaryOperator::NotEqual
            | BinaryOperator::Less
            | BinaryOperator::Greater
            | BinaryOperator::LessEqual
            | BinaryOperator::GreaterEqual
            | BinaryOperator::LogiacalAnd
            | BinaryOperator::LogicalOr => Ok(Type::Bool),
        }
    }
}
