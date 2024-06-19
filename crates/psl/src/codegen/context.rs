use std::{
    cell::{Ref, RefCell},
    fmt::Debug,
    hash::Hash,
    rc::Rc,
};

use super::{construct::Scope, pass::NamesResolved};

pub struct CodegenContext {
    name_resolution: Rc<RefCell<NamesResolved>>,
}

impl CodegenContext {
    pub fn new(resolution: Rc<RefCell<NamesResolved>>) -> CodegenContext {
        CodegenContext {
            name_resolution: resolution,
        }
    }

    pub fn scope<T: Hash + Debug + 'static>(&self, node: &T) -> Ref<Scope> {
        Ref::map(self.name_resolution.borrow(), |root| {
            root.get(node).expect(&format!("{node:?} must have scope"))
        })
    }
}
