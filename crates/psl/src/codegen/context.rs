use std::{
    cell::{Ref, RefCell},
    hash::Hash,
    rc::Rc,
};

use super::{construct::Scope, pass::NamesResolved};

pub struct CodegenContext {
    name_resolution: Rc<RefCell<NamesResolved>>,
    random_index: u64,

    header: String,
    main: String,
    footer: String,
}

impl CodegenContext {
    pub fn new(resolution: Rc<RefCell<NamesResolved>>) -> CodegenContext {
        CodegenContext {
            name_resolution: resolution,

            random_index: 0,

            header: String::new(),
            main: String::new(),
            footer: String::new(),
        }
    }

    pub fn scope<T: Hash + 'static>(&self, node: &T) -> Ref<Scope> {
        Ref::map(self.name_resolution.borrow(), |root| {
            root.get(node).unwrap()
        })
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
