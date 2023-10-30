use crate::{
    ast::{Type, VariableDeclaration},
    codegen::{context::CodegenContext, visitor::CodegenNode},
};

impl CodegenNode for VariableDeclaration {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        let mut output = String::new();
        match self.ty {
            Type::Simple(token) => output.push_str(&token.content),
        }

        output.push_str(" ");
        output.push_str(&self.name.content);

        if let Some(node) = self.value {
            output.push_str(" = ");
            output.push_str(&ctx.visit(node))
        }

        output.push_str(";\n");

        output
    }
}
