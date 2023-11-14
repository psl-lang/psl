use crate::{
    ast::{BinaryOperator, BinaryOperatorExpression},
    codegen::{
        construct::Type,
        context::CodegenContext,
        pass::{NameResolutionContext, NameResolutionPass},
        visitor::CodegenNode,
    },
};

impl CodegenNode for BinaryOperatorExpression {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        if matches!(
            self.operator,
            BinaryOperator::LogiacalAnd | BinaryOperator::LogicalOr
        ) {
            let mut lhs = String::new();
            let output_name = ctx.generate_random_name();
            let ty = self.lhs.infer_type(ctx).unwrap().as_c_type();
            lhs.push_str(&format!(
                "{} {} = {};\n",
                ty,
                &output_name,
                ctx.visit(self.lhs)
            ));

            let should_negate = self.operator == BinaryOperator::LogicalOr;

            ctx.push_main(&lhs);

            ctx.push_main(&format!(
                "if ({}{}) {{\n",
                if should_negate { "!" } else { "" },
                &output_name,
            ));

            let rhs = ctx.visit(self.rhs);
            ctx.push_main(&format!("{} = {};\n", &output_name, rhs));

            ctx.push_main("}\n");

            return output_name;
        }

        let mut output = String::new();
        output.push('(');
        output.push_str(&ctx.visit(self.lhs));
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
        output.push_str(&ctx.visit(self.rhs));
        output.push(')');

        output
    }
}

impl NameResolutionPass for BinaryOperatorExpression {
    fn resolve(&self, ctx: &mut NameResolutionContext) {
        ctx.visit(&self.lhs);
        ctx.visit(&self.rhs);
    }
}

impl BinaryOperatorExpression {
    /**
     * @TODO:
     * when we introduce operator overloading, we should change this.
     */
    pub fn infer_type(&self, ctx: &CodegenContext) -> Result<Type, String> {
        match self.operator {
            BinaryOperator::Add
            | BinaryOperator::Subtract
            | BinaryOperator::Multiply
            | BinaryOperator::Divide
            | BinaryOperator::Modulus => self.lhs.infer_type(ctx),

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
