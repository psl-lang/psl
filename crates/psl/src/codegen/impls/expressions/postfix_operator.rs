use crate::{
    ast::{PostfixOperator, PostfixOperatorExpression},
    codegen::{
        construct::Type,
        context::CodegenContext,
        pass::{NameResolutionContext, NameResolutionPass},
        visitor::CodegenNode,
    },
};

impl CodegenNode for PostfixOperatorExpression {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        match self.operator {
            PostfixOperator::Invoke(arguments) => {
                let mut output = String::new();
                output.push_str(&ctx.visit(self.expr));
                output.push('(');
                for (i, arg) in arguments.into_iter().enumerate() {
                    if i != 0 {
                        output.push_str(", ");
                    }
                    output.push_str(&ctx.visit(arg));
                }
                output.push(')');

                output
            }
            PostfixOperator::Index(_) => todo!(),
        }
    }
}

impl NameResolutionPass for PostfixOperatorExpression {
    fn resolve(&self, ctx: &mut NameResolutionContext) {
        ctx.visit(&self.expr);
        match &self.operator {
            PostfixOperator::Invoke(arguments) => {
                for arg in arguments {
                    ctx.create_subscope().visit(arg);
                }
            }
            PostfixOperator::Index(_) => todo!(),
        }
    }
}

impl PostfixOperatorExpression {
    /**
     * @TODO:
     * when we introduce operator overloading, we should change this.
     */
    pub fn infer_type(&self, ctx: &CodegenContext) -> Result<Type, String> {
        match &self.operator {
            PostfixOperator::Invoke(_) => {
                let Ok(Type::Function { returning, .. }) = self.expr.infer_type(ctx) else {
                    Err("you can only invoke function-typed variables")?
                };
                Ok(*returning.clone())
            }
            PostfixOperator::Index(_) => todo!(),
        }
    }
}
