use crate::{
    ast::{ExpressionOrBlock, FunctionDeclaration},
    codegen::{
        construct::Type,
        context::{CodegenContext, Function},
        pass::{NameResolutionContext, NameResolutionPass},
        visitor::CodegenNode,
    },
};

impl CodegenNode for FunctionDeclaration {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        let return_type = Type::try_from(self.return_type).unwrap();

        let mut forward_decl = String::new();
        let mut decl = String::new();

        forward_decl.push_str(&return_type.as_c_type());
        forward_decl.push(' ');
        forward_decl.push_str(&self.name.content);
        forward_decl.push('(');
        for (i, (param_ty, param_name)) in self.parameters.iter().enumerate() {
            if i != 0 {
                forward_decl.push_str(", ");
            }
            forward_decl.push_str(&Type::try_from(param_ty.clone()).unwrap().as_c_type());
            forward_decl.push(' ');
            forward_decl.push_str(&param_name.content);
        }
        forward_decl.push(')');

        decl.push_str(&forward_decl);
        decl.push_str(" {\n");

        match self.body {
            ExpressionOrBlock::Expression(expr) => {
                decl.push_str("return ");
                decl.push_str(&ctx.visit(expr));
                decl.push_str(";\n");
            }
            ExpressionOrBlock::Block(block) => {
                for statement in block.statements {
                    decl.push_str(&ctx.visit(statement));
                }
                match block.last_expression {
                    Some(last_expr) => {
                        if !matches!(last_expr.infer_type(ctx).unwrap(), Type::Never) {
                            decl.push_str("return ");
                        }
                        decl.push_str(&ctx.visit(last_expr));
                        decl.push_str(";\n");
                    }
                    None => decl.push_str("return {};\n"),
                }
            }
        }
        decl.push('}');

        ctx.add_function(Function {
            forward_decl: format!("{forward_decl};"),
            decl,
        });

        "".to_owned()
    }
}

impl NameResolutionPass for FunctionDeclaration {
    fn resolve(&self, ctx: &mut NameResolutionContext) {
        let ty = Type::Function {
            parameters: self
                .parameters
                .iter()
                .map(|(ty, _)| Type::try_from(ty.clone()).unwrap())
                .collect(),
            returning: Box::new(Type::try_from(self.return_type.clone()).unwrap()),
        };
        ctx.scope_mut().put_variable(self.name.content.clone(), ty);
        ctx.create_subscope().visit(&self.body);
    }
}
