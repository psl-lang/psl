use crate::{
    ast::{ReadExpression, Type},
    codegen::{context::CodegenContext, visitor::CodegenNode},
};

impl CodegenNode for ReadExpression {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        match self {
            ReadExpression::Type(ty) => match ty {
                Type::Simple(token) => format!("__read_{}()", token.content),
            },
        }
    }
}
