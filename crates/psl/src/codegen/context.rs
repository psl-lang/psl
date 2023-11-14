use std::{cell::Ref, fmt::Debug, hash::Hash};

use super::{construct::Scope, pass::NamesResolved};

pub struct CodegenContext {
    name_resolution: NamesResolved,
    random_index: u64,

    header: String,
    main: String,
    footer: String,
}

impl CodegenContext {
    pub fn new(resolution: NamesResolved) -> CodegenContext {
        CodegenContext {
            name_resolution: resolution,

            random_index: 0,

            header: String::new(),
            main: String::new(),
            footer: String::new(),
        }
    }

    pub fn scope<T: Debug + Hash + 'static>(&self, node: &T) -> Ref<Scope> {
        self.name_resolution
            .get(node)
            .expect(&format!("{:#?} has no scope", &node))
            .borrow()
    }

    pub fn generate_random_name(&mut self) -> String {
        let result = format!("__psl_random_name_{}", self.random_index);
        self.random_index += 1;
        result
    }

    pub fn push_header(&mut self, s: &str) {
        self.header.push_str(s)
    }

    pub fn push_main(&mut self, s: &str) {
        self.main.push_str(s)
    }

    pub fn push_footer(&mut self, s: &str) {
        self.footer.push_str(s)
    }

    pub fn output(self) -> String {
        format!("{}\n{}\n{}", self.header, self.main, self.footer)
    }
}
