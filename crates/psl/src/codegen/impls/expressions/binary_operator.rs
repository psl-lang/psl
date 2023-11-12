use crate::{
    ast::{BinaryOperator, BinaryOperatorExpression},
    codegen::{construct::Type, context::CodegenContext, visitor::CodegenNode},
};

impl CodegenNode for BinaryOperatorExpression {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
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
            BinaryOperator::LogiacalAnd => "&&",
            BinaryOperator::LogicalOr => "||",
        };

        output.push_str(operator);

        output.push('(');
        output.push_str(&ctx.visit(self.rhs));
        output.push(')');

        output
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
