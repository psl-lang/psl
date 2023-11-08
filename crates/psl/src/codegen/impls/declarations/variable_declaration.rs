use crate::{
    ast::{Type, VariableDeclaration},
    codegen::{context::CodegenContext, visitor::CodegenNode},
};

impl CodegenNode for VariableDeclaration {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        ctx.add_variable(self.name.content.clone(), self.ty.clone());

        let mut output = String::new();
        match self.ty {
            Type::Simple(token) => output.push_str(&token.content),
        }

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
