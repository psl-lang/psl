use crate::{
    ast::NameExpression,
    codegen::{construct::Type, context::CodegenContext, visitor::CodegenNode},
};

impl CodegenNode for NameExpression {
    fn produce_code(self, _ctx: &mut CodegenContext) -> String {
        self.name.content
    }
}

impl NameExpression {
    pub fn infer_type(&self, ctx: &CodegenContext) -> Result<Type, String> {
        ctx.get_variable_type(&self.name.content)
            .cloned()
            .ok_or_else(|| format!("There is no variable named {:?}", self.name.content))
    }
}
