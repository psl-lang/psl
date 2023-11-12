use crate::{
    ast::WriteStatement,
    codegen::{context::CodegenContext, scope::Scope, visitor::CodegenNode},
};

impl CodegenNode for WriteStatement {
    fn produce_code(self, ctx: &mut CodegenContext, scope: &mut Scope) -> String {
        let Some(ty) = scope.get_variable_type(&self.name.name.content) else {
            // @TODO: migrate to diagnostics
            panic!("There is no variable named {:?}", self.name.name.content);
        };

        ctx.push_main(&format!(
            "__write_{}(write_buf, {});\n",
            ty.as_c_type(),
            self.name.name.content
        ));

        "".to_owned()
    }
}
