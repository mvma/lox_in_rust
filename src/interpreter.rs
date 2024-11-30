use std::borrow::BorrowMut;

use crate::{environment::*, Expression, Token};

pub enum Statement {
    Expression {
        expression: Expression,
    },
    Print {
        expression: Expression,
    },
    Var {
        token: Token,
        expression: Expression,
    },
    Block {
        statements: Vec<Statement>,
    },
}

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new(environment: Environment) -> Self {
        Self { environment }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            self.execute(statement);
        }
    }

    fn execute(&mut self, statement: Statement) {
        match statement {
            Statement::Print { expression } => {
                let value = expression.evaluate(&mut self.environment);
                println!("{:#?}", value.to_string());
            }
            Statement::Expression { expression } => {
                expression.evaluate(&mut self.environment);
            }
            Statement::Var { token, expression } => {
                let value = expression.evaluate(&mut self.environment);

                self.environment.define(token.lexeme(), value);
            }
            Statement::Block { statements } => {
                let previous = self.environment.borrow_mut().clone();

                let current =
                    Environment::new_with_enclosing(Some(Box::new(self.environment.clone())));

                self.environment = current;
                self.interpret(statements);
                self.environment = previous;
            }
        }
    }
}
