use crate::{
    ast::{FormatSpecifierFragment, WriteStatement},
    codegen::{
        context::CodegenContext,
        pass::{NameResolutionContext, NameResolutionPass},
        visitor::CodegenNode,
    },
};

impl CodegenNode for WriteStatement {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        let mut output = String::new();
        for frag in &self.fragments {
            match frag {
                FormatSpecifierFragment::Text(s) | FormatSpecifierFragment::Whitespace(s) => {
                    let bytes = s.as_bytes();
                    output.push_str(&format!(
                        "__sys_write(write_buf, \"{}\", {});\n",
                        bytes
                            .iter()
                            .map(|b| format!("\\x{:x}", b))
                            .collect::<String>(),
                        bytes.len()
                    ));
                }
                FormatSpecifierFragment::Variable(var) => {
                    let Some(ty) = ctx.scope(&self).get_variable_type(&var) else {
                        // @TODO: migrate to diagnostics
                        panic!("There is no variable named {:?}", var);
                    };

                    output.push_str(&format!(
                        "__write_{}(write_buf, {});\n",
                        ty.as_c_type(),
                        var
                    ));
                }
            }
        }
        output
    }
}

impl NameResolutionPass for WriteStatement {
    fn resolve(&self, _ctx: &mut NameResolutionContext) {}
}
