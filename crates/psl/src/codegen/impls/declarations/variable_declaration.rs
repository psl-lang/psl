use crate::{
    ast::VariableDeclaration,
    codegen::{construct::Type, context::CodegenContext, scope::Scope, visitor::CodegenNode},
};

impl CodegenNode for VariableDeclaration {
    fn produce_code(self, ctx: &mut CodegenContext, scope: &mut Scope) -> String {
        let ty = Type::try_from(self.ty).unwrap();
        scope.add_variable(self.name.content.clone(), ty.clone());

        let mut output = String::new();

        output.push_str(&ty.as_c_type());
        output.push(' ');
        output.push_str(&self.name.content);

        if let Some(node) = self.value {
            output.push('=');
            output.push_str(&node.produce_code(ctx, scope))
        }

        output.push_str(";\n");

        ctx.push_main(&output);

        "".to_owned()
    }
}
