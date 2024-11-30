use std::collections::HashMap;

use crate::Literal;

#[derive(Clone)]
pub struct Environment {
    variables: HashMap<String, Literal>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_with_enclosing(environment: Option<Box<Environment>>) -> Self {
        Self {
            variables: HashMap::new(),
            enclosing: environment,
        }
    }

    pub fn get(&self, variable_name: &str) -> Option<&Literal> {
        let value = self.variables.get(variable_name);

        match (value, &self.enclosing) {
            (Some(value), _) => Some(value),
            (None, Some(enclosing)) => enclosing.get(variable_name),
            _ => panic!("Undefined variable '{}'.", variable_name),
        }
    }

    pub fn define(&mut self, variable_name: &str, value: Literal) {
        self.variables.insert(variable_name.to_string(), value);
    }

    pub fn assign(&mut self, variable_name: &str, value: Literal) -> bool {
        let previous_value = self.variables.get(variable_name);

        match (previous_value, &mut self.enclosing) {
            (Some(_), _) => {
                self.variables.insert(variable_name.to_string(), value);

                true
            }
            (None, Some(next)) => next.assign(variable_name, value),
            (None, None) => false,
        }
    }
}
