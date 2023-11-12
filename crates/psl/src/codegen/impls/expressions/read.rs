use crate::{
    ast::ReadExpression,
    codegen::{construct::Type, context::CodegenContext, scope::Scope, visitor::CodegenNode},
};

impl CodegenNode for ReadExpression {
    fn produce_code(self, _ctx: &mut CodegenContext, scope: &mut Scope) -> String {
        match self {
            ReadExpression::Type(ty) => {
                let ty = Type::try_from(ty).unwrap();
                format!("__read_{}(read_buf)", ty.as_c_type())
            }
        }
    }
}

impl ReadExpression {
    pub fn infer_type(&self, _ctx: &CodegenContext, scope: &mut Scope) -> Result<Type, String> {
        match self {
            ReadExpression::Type(ty) => ty.clone().try_into(),
        }
    }
}
