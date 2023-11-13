use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::Type;

pub struct Scope {
    parent: Option<Rc<RefCell<Scope>>>,

    variable_names: HashMap<String, Type>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            parent: None,

            variable_names: HashMap::new(),
        }
    }

    pub fn create_subscope(scope: Rc<RefCell<Scope>>) -> Scope {
        Scope {
            parent: Some(scope.clone()),

            variable_names: HashMap::new(),
        }
    }

    pub fn put_variable(&mut self, name: String, ty: Type) {
        self.variable_names.insert(name, ty);
    }

    pub fn get_variable_type(&self, name: &String) -> Option<Type> {
        self.variable_names.get(name).cloned().or_else(|| {
            self.parent
                .as_deref()
                .and_then(|parent| parent.borrow().get_variable_type(name))
        })
    }
}
