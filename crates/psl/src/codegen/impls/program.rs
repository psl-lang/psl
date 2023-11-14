use crate::{
    ast::Program,
    codegen::{
        context::CodegenContext,
        pass::{NameResolutionContext, NameResolutionPass},
        visitor::CodegenNode,
    },
};

macro_rules! include_rt {
    ($ctx:expr, $file:literal) => {
        $ctx.push_header(concat!("// ", $file, "\n"));
        $ctx.push_header(&remove_c_preprocessor_codes(include_str!(concat!(
            "../rt/", $file
        ))));
        $ctx.push_header("\n\n")
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
        ctx.push_header(
            &include_str!("../rt/header.h")
                .replace("{{CARGO_PKG_VERSION}}", env!("CARGO_PKG_VERSION")),
        );
        ctx.push_header("#include <unistd.h>\n");
        ctx.push_header("#include <stddef.h>\n");
        ctx.push_header("#include <stdbool.h>\n");
        ctx.push_header("#include <stdint.h>\n");
        ctx.push_header("\n");
        include_rt!(ctx, "typedef.h");
        include_rt!(ctx, "write.h");
        include_rt!(ctx, "panic.h");
        include_rt!(ctx, "read.h");

        ctx.push_header("int main() {}; int __libc_start_main() {\n");
        ctx.push_header("c8 read_buf[READ_BUF_LEN];\n");
        ctx.push_header("c8 write_buf[WRITE_BUF_LEN];\n");

        for item in self.items {
            let output = ctx.visit(item);
            ctx.push_main(&output);
        }

        ctx.push_footer("__flush(write_buf);\n");
        ctx.push_footer("_Exit(0);\n");
        ctx.push_footer("}\n");

        "".to_owned()
    }
}

impl NameResolutionPass for Program {
    fn resolve(&self, ctx: &mut NameResolutionContext) {
        for item in &self.items {
            ctx.visit(item)
        }
    }
}
