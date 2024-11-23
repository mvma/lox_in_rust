
use crate::{
    environment::{*},
    Expression, Token,
};

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
}

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new(environment: Environment) -> Self {
        Self {
            environment: environment,
        }
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

                self.environment.set(token.lexeme().to_string(), value);
            }
        }
    }
}
