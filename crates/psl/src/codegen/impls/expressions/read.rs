use crate::{
    ast::ReadExpression,
    codegen::{
        construct::Type,
        context::CodegenContext,
        pass::{NameResolutionContext, NameResolutionPass},
        visitor::CodegenNode,
    },
};

impl CodegenNode for ReadExpression {
    fn produce_code(self, _ctx: &mut CodegenContext) -> String {
        match self {
            ReadExpression::Type(ty) => {
                let ty = Type::try_from(ty).unwrap();
                format!("__read_{}(read_buf)", ty.as_c_type())
            }
        }
    }
}

impl NameResolutionPass for ReadExpression {
    fn resolve(&self, _ctx: &mut NameResolutionContext) {}
}

impl ReadExpression {
    pub fn infer_type(&self, _ctx: &CodegenContext) -> Result<Type, String> {
        match self {
            ReadExpression::Type(ty) => ty.clone().try_into(),
        }
    }
}
