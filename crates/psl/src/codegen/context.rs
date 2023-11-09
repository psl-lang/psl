use std::collections::HashMap;

use super::construct::Type;

pub struct CodegenContext {
    variable_names: HashMap<String, Type>,
}

impl CodegenContext {
    pub fn new() -> CodegenContext {
        CodegenContext {
            variable_names: HashMap::new(),
        }
    }

    pub fn add_variable(&mut self, name: String, ty: Type) {
        self.variable_names.insert(name, ty);
    }

    pub fn get_variable_type(&self, name: &String) -> Option<&Type> {
        self.variable_names.get(name)
    }
}
