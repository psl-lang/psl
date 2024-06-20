use std::{
    cell::{Ref, RefCell},
    fmt::Debug,
    hash::Hash,
    rc::Rc,
};

use super::{construct::Scope, pass::NamesResolved};

pub struct CodegenContext {
    functions: Vec<Function>,
    name_resolution: Rc<RefCell<NamesResolved>>,
}

pub struct Function {
    pub forward_decl: String,
    pub decl: String,
}

impl CodegenContext {
    pub fn new(resolution: Rc<RefCell<NamesResolved>>) -> CodegenContext {
        CodegenContext {
            functions: Vec::new(),
            name_resolution: resolution,
        }
    }

    pub fn scope<T: Hash + Debug + 'static>(&self, node: &T) -> Ref<Scope> {
        Ref::map(self.name_resolution.borrow(), |root| {
            root.get(node).expect(&format!("{node:?} must have scope"))
        })
    }

    pub fn add_function(&mut self, f: Function) {
        self.functions.push(f)
    }

    pub fn func_forward_decls(&self) -> impl Iterator<Item = &String> {
        self.functions.iter().map(|f| &f.forward_decl)
    }

    pub fn func_decls(&self) -> impl Iterator<Item = &String> {
        self.functions.iter().map(|f| &f.decl)
    }
}
