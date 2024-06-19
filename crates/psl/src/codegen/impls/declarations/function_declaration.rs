use crate::{
    ast::{ExpressionOrBlock, FunctionDeclaration},
    codegen::{
        construct::Type,
        context::CodegenContext,
        pass::{NameResolutionContext, NameResolutionPass},
        visitor::CodegenNode,
    },
};

impl CodegenNode for FunctionDeclaration {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        let return_type = Type::try_from(self.return_type).unwrap();
        let mut output = String::new();

        output.push_str(&return_type.as_c_type());
        output.push(' ');
        output.push_str(&self.name.content);
        output.push('(');
        for (i, (param_ty, param_name)) in self.parameters.iter().enumerate() {
            if i != 0 {
                output.push_str(", ");
            }
            output.push_str(&Type::try_from(param_ty.clone()).unwrap().as_c_type());
            output.push(' ');
            output.push_str(&param_name.content);
        }
        output.push_str(") {\n");

        match self.body {
            ExpressionOrBlock::Expression(expr) => {
                output.push_str("return ");
                output.push_str(&ctx.visit(expr));
                output.push_str(";\n");
            }
            ExpressionOrBlock::Block(block) => {
                for statement in block.statements {
                    output.push_str(&ctx.visit(statement));
                }
                match block.last_expression {
                    Some(last_expr) => {
                        if !matches!(last_expr.infer_type(ctx).unwrap(), Type::Never) {
                            output.push_str("return ");
                        }
                        output.push_str(&ctx.visit(last_expr));
                        output.push_str(";\n");
                    }
                    None => output.push_str("return {};\n"),
                }
            }
        }
        output.push('}');

        output
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
