use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::codegen::pass::NamesResolved;

use super::Type;

pub struct Scope {
    root: Rc<RefCell<NamesResolved>>,

    parent: Option<usize>,

    variable_names: HashMap<String, Type>,
}

impl Scope {
    pub fn new(root: Rc<RefCell<NamesResolved>>) -> Scope {
        Scope {
            root,
            parent: None,
            variable_names: HashMap::new(),
        }
    }

    pub fn with_parentt(root: Rc<RefCell<NamesResolved>>, parent: usize) -> Scope {
        Scope {
            root,
            parent: Some(parent),

            variable_names: HashMap::new(),
        }
    }

    pub fn put_variable(&mut self, name: String, ty: Type) {
        self.variable_names.insert(name, ty);
    }

    pub fn get_variable_type(&self, name: &String) -> Option<Type> {
        self.variable_names.get(name).cloned().or_else(|| {
            self.parent
                .and_then(|parent| self.root.borrow().vec[parent].get_variable_type(name))
        })
    }
}
