use crate::{
    ast::{BinaryOperator, BinaryOperatorExpression},
    codegen::{context::CodegenContext, visitor::CodegenNode},
};

impl CodegenNode for BinaryOperatorExpression {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        let mut output = String::new();

        output.push_str("(");
        output.push_str(&ctx.visit(self.lhs));
        output.push_str(")");

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
            BinaryOperator::LogiacalAnd => "&&",
            BinaryOperator::LogicalOr => "||",
        };

        output.push_str(operator);

        output.push_str("(");
        output.push_str(&ctx.visit(self.rhs));
        output.push_str(")");

        output
    }
}
