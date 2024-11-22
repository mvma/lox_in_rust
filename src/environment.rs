use std::collections::HashMap;

use crate::Literal;

pub struct Environment {
    variables: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn get(&self, variable_name: &str) -> Option<&Literal> {
        self.variables.get(variable_name)
    }

    pub fn set(&mut self, variable_name: String, value: Literal) {
        self.variables.insert(variable_name, value);
    }
}
