use crate::{
    ast::CompoundAssignmentStatement,
    codegen::{
        context::CodegenContext,
        pass::{NameResolutionContext, NameResolutionPass},
        visitor::CodegenNode,
    },
};

impl CodegenNode for CompoundAssignmentStatement {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        let (assignment, operator) = match self {
            CompoundAssignmentStatement::Simple(assignment) => (assignment, "="),
        };

        format!(
            "{} {} {};\n",
            assignment.name.content,
            operator,
            ctx.visit(assignment.expr)
        )
    }
}

impl NameResolutionPass for CompoundAssignmentStatement {
    fn resolve(&self, _ctx: &mut NameResolutionContext) {}
}
