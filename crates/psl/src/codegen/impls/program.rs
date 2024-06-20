use crate::{
    ast::Program,
    codegen::{
        context::CodegenContext,
        pass::{NameResolutionContext, NameResolutionPass},
        visitor::CodegenNode,
    },
};

macro_rules! include_rt {
    ($output:expr, $file:literal) => {
        $output.push_str(concat!("// ", $file, "\n"));
        $output.push_str(&remove_c_preprocessor_codes(include_str!(concat!(
            "../rt/", $file
        ))));
        $output.push_str("\n\n")
    };
}

fn remove_c_preprocessor_codes(s: impl AsRef<str>) -> String {
    s.as_ref()
        .split('\n')
        .filter(|s| !s.is_empty() && &s[0..1] != "#") // do not support C preprocessor at all
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_owned()
}

impl CodegenNode for Program {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        let mut main_body = String::new();

        for item in self.items {
            main_body.push_str(&ctx.visit(item));
        }

        let mut output = String::new();
        output.push_str(
            &include_str!("../rt/header.h")
                .replace("{{CARGO_PKG_VERSION}}", env!("CARGO_PKG_VERSION")),
        );
        output.push_str("#include <unistd.h>\n");
        output.push_str("#include <stddef.h>\n");
        output.push_str("#include <stdbool.h>\n");
        output.push_str("#include <stdint.h>\n");
        output.push('\n');
        include_rt!(output, "typedef.h");
        include_rt!(output, "write.h");
        include_rt!(output, "panic.h");
        include_rt!(output, "read.h");

        for f in ctx.func_forward_decls() {
            output.push_str(&f);
        }

        output.push_str("int main() {}; int __libc_start_main() {\n");
        output.push_str("c8 read_buf[READ_BUF_LEN];\n");
        output.push_str("c8 write_buf[WRITE_BUF_LEN];\n");

        output.push_str(&main_body);

        output.push_str("__flush(write_buf);\n");
        output.push_str("_Exit(0);\n");
        output.push_str("}\n");

        for f in ctx.func_decls() {
            output.push_str(&f);
        }

        output
    }
}

impl NameResolutionPass for Program {
    fn resolve(&self, ctx: &mut NameResolutionContext) {
        for item in &self.items {
            ctx.visit(item)
        }
    }
}
