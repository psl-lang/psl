use crate::{
    ast::NameExpression,
    codegen::{
        construct::Type,
        context::CodegenContext,
        pass::{NameResolutionContext, NameResolutionPass},
        visitor::CodegenNode,
    },
};

impl CodegenNode for NameExpression {
    fn produce_code(self, _ctx: &mut CodegenContext) -> String {
        self.name.content
    }
}

impl NameResolutionPass for NameExpression {
    fn resolve(&self, _ctx: &mut NameResolutionContext) {}
}

impl NameExpression {
    pub fn infer_type(&self, ctx: &CodegenContext) -> Result<Type, String> {
        ctx.scope(self)
            .get_variable_type(&self.name.content)
            .ok_or_else(|| format!("There is no variable named {:?}", self.name.content))
    }
}
