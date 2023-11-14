use std::hash::Hash;

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

    pub fn scope<T: Hash + 'static>(&self, node: &T) -> &Scope {
        self.name_resolution.get(node).unwrap()
    }
}
