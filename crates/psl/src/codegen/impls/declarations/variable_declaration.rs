use crate::{
    ast::VariableDeclaration,
    codegen::{construct::Type, context::CodegenContext, visitor::CodegenNode},
};

impl CodegenNode for VariableDeclaration {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        let ty = Type::try_from(self.ty).unwrap();
        ctx.add_variable(self.name.content.clone(), ty.clone());

        let mut output = String::new();

        output.push_str(&ty.as_c_type());
        output.push(' ');
        output.push_str(&self.name.content);

        if let Some(node) = self.value {
            output.push('=');
            output.push_str(&ctx.visit(node))
        }

        output.push_str(";\n");

        output
    }
}
