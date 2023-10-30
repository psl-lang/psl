use crate::{
    ast::{BinaryOperator, BinaryOperatorExpression},
    codegen::{context::CodegenContext, visitor::CodegenNode},
};

impl CodegenNode for BinaryOperatorExpression {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        let mut output = String::new();

        output.push_str(&ctx.visit(self.lhs));

        /*
         * @TODO:
         * when we introduce operator overloading, we should change this.
         */
        let operator = match self.operator {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
        };

        output.push_str(operator);

        output.push_str(&ctx.visit(self.rhs));

        output
    }
}
