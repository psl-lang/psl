use std::{
    cell::{Ref, RefCell},
    hash::Hash,
    rc::Rc,
};

use super::{construct::Scope, pass::NamesResolved};

pub struct CodegenContext {
    name_resolution: NamesResolved,
}

impl CodegenContext {
    pub fn new(resolution: NamesResolved) -> CodegenContext {
        CodegenContext {
            name_resolution: resolution,
        }
    }

    pub fn scope<T: Hash + 'static>(&self, node: &T) -> Ref<Scope> {
        self.name_resolution.get(node).unwrap().borrow()
    }
}
