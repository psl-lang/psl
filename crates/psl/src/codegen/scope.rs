use std::collections::HashMap;

use super::construct::Type;

#[derive(Debug)]
pub struct Scope<'a> {
    parent: Option<&'a Scope<'a>>,
    variable_names: HashMap<String, Type>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Scope<'static> {
        Scope {
            parent: None,
            variable_names: HashMap::new(),
        }
    }

    pub fn create_subscope<'b>(&'b mut self) -> Scope<'b> {
        Scope {
            parent: Some(self),
            variable_names: HashMap::new(),
        }
    }

    pub fn add_variable(&mut self, name: String, ty: Type) {
        self.variable_names.insert(name, ty);
    }

    pub fn get_variable_type(&self, name: &String) -> Option<&Type> {
        self.variable_names.get(name).or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.get_variable_type(name))
        })
    }
}
