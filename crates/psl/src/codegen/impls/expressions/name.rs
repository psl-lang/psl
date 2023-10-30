use crate::{
    ast::NameExpression,
    codegen::{context::CodegenContext, visitor::CodegenNode},
};

impl CodegenNode for NameExpression {
    fn produce_code(self, _ctx: &mut CodegenContext) -> String {
        self.name.content
    }
}
