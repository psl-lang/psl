use crate::{
    ast::NameExpression,
    codegen::{construct::Type, context::CodegenContext, scope::Scope, visitor::CodegenNode},
};

impl CodegenNode for NameExpression {
    fn produce_code(self, _ctx: &mut CodegenContext, scope: &mut Scope) -> String {
        self.name.content
    }
}

impl NameExpression {
    pub fn infer_type(&self, ctx: &CodegenContext, scope: &mut Scope) -> Result<Type, String> {
        scope
            .get_variable_type(&self.name.content)
            .cloned()
            .ok_or_else(|| format!("There is no variable named {:?}", self.name.content))
    }
}
