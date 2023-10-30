use crate::{ast::Program, codegen::visitor::CodegenNode};

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
    fn produce_code(self, ctx: &mut crate::codegen::context::CodegenContext) -> String {
        let mut output = String::new();
        output.push_str("#pragma once\n");
        output.push_str("#inclue <unistd.h>\n");
        output.push_str("#inclue <stddef.h>\n");
        output.push_str("#inclue <stdbool.h>\n");
        output.push_str("#inclue <stdint.h>\n");
        output.push_str("\n");
        include_rt!(output, "typedef.h");
        include_rt!(output, "write.h");

        output.push_str("int __libc_start_main() {\n");
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
