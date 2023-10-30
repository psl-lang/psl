use crate::{
    ast::Program,
    codegen::{context::CodegenContext, visitor::CodegenNode},
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

impl CodegenNode for Program {
    fn produce_code(self, ctx: &mut CodegenContext) -> String {
        let mut output = String::new();
        output.push_str("#include <unistd.h>\n");
        output.push_str("#include <stddef.h>\n");
        output.push_str("#include <stdbool.h>\n");
        output.push_str("#include <stdint.h>\n");
        output.push_str("\n");
        include_rt!(output, "typedef.h");
        include_rt!(output, "write.h");
        include_rt!(output, "panic.h");

        output.push_str("int main; int __libc_start_main() {\n");

        for item in self.items {
            output.push_str(&ctx.visit(item));
        }

        output.push_str("}\n");

        output
    }
}

fn remove_c_preprocessor_codes(s: impl AsRef<str>) -> String {
    s.as_ref()
        .split("\n")
        .filter(|s| !s.is_empty() && &s[0..1] != "#") // do not support C preprocessor at all
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_owned()
}
