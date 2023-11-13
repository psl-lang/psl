use crate::{
    ast::VariableDeclaration,
    codegen::{
        construct::Type,
        context::CodegenContext,
        pass::{NameResolutionContext, NameResolutionPass},
        visitor::CodegenNode,
    },
};

impl CodegenNode for VariableDeclaration {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        let ty = Type::try_from(self.ty).unwrap();
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

impl NameResolutionPass for VariableDeclaration {
    fn resolve(&self, ctx: &mut NameResolutionContext) {
        let ty = Type::try_from(self.ty.clone()).unwrap();
        ctx.scope_mut().put_variable(self.name.content.clone(), ty);
    }
}
