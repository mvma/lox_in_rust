use std::str;

use crate::Expression;

pub enum Statement {
    Expression { expression: Expression },
    Print { expression: Expression },
}

pub struct Interpreter {
    statements: Vec<Statement>,
}

impl Interpreter {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self {
            statements: statements,
        }
    }

    pub fn interpret(&self) {
        for statement in &self.statements {
            self.execute(statement);
        }
    }

    fn execute(&self, statement: &Statement) {
        match statement {
            Statement::Print { expression } => {
                let value = expression.evaluate();
                println!("{:#?}", value.to_string());
            }
            Statement::Expression { expression } => {
                expression.evaluate();
            }
        }
    }
}
