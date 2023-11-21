use crate::{
    ast::WhileStatement,
    codegen::{
        context::CodegenContext,
        pass::{NameResolutionContext, NameResolutionPass},
        visitor::CodegenNode,
    },
};

impl CodegenNode for WhileStatement {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        let mut output = String::new();

        output.push_str("while (");
        output.push_str(&ctx.visit(self.condition));
        output.push_str(") {\n");
        for statement in self.block.statements {
            output.push_str(&ctx.visit(statement));
        }
        if let Some(last_expression) = self.block.last_expression {
            output.push_str(&format!("{};\n", ctx.visit(last_expression)));
        }
        output.push_str("}\n");

        output
    }
}

impl NameResolutionPass for WhileStatement {
    fn resolve(&self, ctx: &mut NameResolutionContext) {
        let mut scope = ctx.create_subscope();
        scope.visit(&self.condition);
        for statement in &self.block.statements {
            ctx.visit(statement);
        }
        ctx.visit(&self.block.last_expression)
    }
}
