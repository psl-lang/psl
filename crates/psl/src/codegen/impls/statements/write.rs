use crate::{
    ast::{Type, WriteStatement},
    codegen::{context::CodegenContext, visitor::CodegenNode},
};

impl CodegenNode for WriteStatement {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        let Some(ty) = ctx.get_variable_type(&self.name.name.content) else {
            // @TODO: migrate to diagnostics
            eprintln!("Cannot compile write statement");
            return "".to_owned()
        };

        let ty = match ty {
            Type::Simple(token) => &token.content,
        };

        format!("__write_{}(write_buf, {});\n", ty, self.name.name.content)
    }
}
