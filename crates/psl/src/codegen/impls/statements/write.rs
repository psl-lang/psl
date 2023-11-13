use crate::{
    ast::WriteStatement,
    codegen::{
        context::CodegenContext,
        pass::{NameResolutionContext, NameResolutionPass},
        visitor::CodegenNode,
    },
};

impl CodegenNode for WriteStatement {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        let Some(ty) = ctx.scope(&self).get_variable_type(&self.name.name.content) else {
            // @TODO: migrate to diagnostics
            panic!("There is no variable named {:?}", self.name.name.content);
        };

        format!(
            "__write_{}(write_buf, {});\n",
            ty.as_c_type(),
            self.name.name.content
        )
    }
}

impl NameResolutionPass for WriteStatement {
    fn resolve(&self, _ctx: &mut NameResolutionContext) {}
}
