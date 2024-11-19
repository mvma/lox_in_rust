use crate::scanner::*;

#[derive(PartialEq)]
pub enum Expression {
    Grouping {
        expression: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Literal {
        literal_value: Literal,
    },
}

pub enum Statement {
    Expression { expression: Expression },
    Print { expression: Expression },
}

impl Literal {
    pub fn to_string(&self) -> String {
        match self {
            Literal::Number(value) => format!("{}", value),
            Literal::Boolean(value) => format!("{}", value),
            Literal::Text(value) => value.clone(),
            Literal::Nil => "nil".to_string(),
        }
    }
}

impl Expression {
    pub fn to_custom_string(&self) -> String {
        match self {
            Expression::Grouping { expression } => {
                format!("(group {})", expression.to_custom_string())
            }
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                format!(
                    "({} {} {})",
                    operator.lexeme(),
                    left.to_custom_string(),
                    right.to_custom_string()
                )
            }
            Expression::Unary { operator, right } => {
                format!("({} {})", operator.lexeme(), right.to_custom_string())
            }
            Expression::Literal { literal_value } => {
                format!("{}", literal_value.to_custom_string())
            }
        }
    }

    pub fn evaluate_and_print(&self) {
        let result = self.evaluate();
        println!("{}", result.to_string());
    }

    pub fn evaluate(&self) -> Literal {
        match self {
            Expression::Grouping { expression } => expression.evaluate(),
            Expression::Binary {
                left,
                operator,
                right,
            } => self.evaluate_binary(left, operator, right),
            Expression::Unary { operator, right } => self.evaluate_unary(operator, right),
            Expression::Literal { literal_value } => literal_value.clone(),
        }
    }

    fn evaluate_binary(
        &self,
        left: &Box<Expression>,
        token: &Token,
        right: &Box<Expression>,
    ) -> Literal {
        let left_expression = left.evaluate();
        let right_expression = right.evaluate();

        match (&left_expression, token.get_token_type(), &right_expression) {
            (Literal::Number(l), TokenType::Minus, Literal::Number(r)) => Literal::Number(l - r),
            (Literal::Number(l), TokenType::Plus, Literal::Number(r)) => Literal::Number(l + r),
            (Literal::Number(l), TokenType::Star, Literal::Number(r)) => Literal::Number(l * r),
            (Literal::Number(l), TokenType::Slash, Literal::Number(r)) => {
                if *r == 0.0 {
                    panic!("Can't divide by zero");
                }
                Literal::Number(l / r)
            }
            (Literal::Number(l), TokenType::Greater, Literal::Number(r)) => Literal::Boolean(l > r),
            (Literal::Number(l), TokenType::GreaterEqual, Literal::Number(r)) => {
                Literal::Boolean(l >= r)
            }
            (Literal::Number(l), TokenType::Less, Literal::Number(r)) => Literal::Boolean(l < r),
            (Literal::Number(l), TokenType::LessEqual, Literal::Number(r)) => {
                Literal::Boolean(l <= r)
            }
            (Literal::Text(l), TokenType::Plus, Literal::Text(r)) => {
                Literal::Text(format!("{}{}", l, r))
            }
            (_, TokenType::EqualEqual, _) => {
                Literal::Boolean(self.is_equal(left_expression, right_expression))
            }
            (_, TokenType::BangEqual, _) => {
                Literal::Boolean(!self.is_equal(left_expression, right_expression))
            }
            _ => panic!(
                "Unsupported binary operation: {} {} {}",
                left_expression.to_custom_string(),
                token.lexeme(),
                right_expression.to_custom_string()
            ),
        }
    }

    fn evaluate_unary(&self, token: &Token, expression: &Box<Expression>) -> Literal {
        let right_expression = expression.evaluate();

        match (token.get_token_type(), right_expression) {
            (TokenType::Minus, Literal::Number(value)) => return Literal::Number(-value),
            _ => todo!(),
        }
    }

    fn is_equal(&self, left: Literal, right: Literal) -> bool {
        match (left, right) {
            (Literal::Number(l), Literal::Number(r)) => l == r,
            (Literal::Text(l), Literal::Text(r)) => l == r,
            (Literal::Boolean(l), Literal::Boolean(r)) => l == r,
            (Literal::Nil, Literal::Nil) => true,
            _ => {
                panic!("Can't compare 2 different types");
            }
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse_expression(&mut self) -> Expression {
        self.expression()
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements: Vec<Statement> = vec![];

        while !self.is_at_end() {
            statements.push(self.statement());
        }

        statements
    }

    fn statement(&mut self) -> Statement {
        if self.match_any(&[TokenType::Print]) {
            return self.print_statement();
        }

        self.expression_statement()
    }

    fn print_statement(&mut self) -> Statement {
        let expression = self.expression();

        self.consume(
            &TokenType::Semicolon,
            "Expect ';' after expression.".to_string(),
        );

        Statement::Print {
            expression: (expression),
        }
    }

    fn expression_statement(&mut self) -> Statement {
        let expression = self.expression();

        self.consume(
            &TokenType::Semicolon,
            "Expect ';' after expression.".to_string(),
        );

        Statement::Expression {
            expression: (expression),
        }
    }

    // expression     → ...
    // equality       → ...
    // comparison     → ...
    // term           → ...
    // factor         → ...
    // unary          → ...
    // primary        → ...
    fn expression(&mut self) -> Expression {
        self.equality()
    }

    fn equality(&mut self) -> Expression {
        let mut expr = self.comparison();

        while self.match_any(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();

            let right_expression = self.comparison();

            expr = Expression::Binary {
                left: (Box::from(expr)),
                operator: (operator),
                right: (Box::from(right_expression)),
            }
        }

        expr
    }

    fn comparison(&mut self) -> Expression {
        let mut expr = self.term();

        while self.match_any(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();

            let right_expression: Expression = self.term();

            expr = Expression::Binary {
                left: (Box::from(expr)),
                operator: (operator),
                right: (Box::from(right_expression)),
            };
        }

        expr
    }

    fn term(&mut self) -> Expression {
        let mut expr: Expression = self.factor();

        while self.match_any(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();

            let right_expression: Expression = self.factor();

            expr = Expression::Binary {
                left: (Box::from(expr)),
                operator: (operator),
                right: (Box::from(right_expression)),
            };
        }

        expr
    }

    fn factor(&mut self) -> Expression {
        let mut expr: Expression = self.unary();

        while self.match_any(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();

            let right_expression: Expression = self.unary();

            expr = Expression::Binary {
                left: (Box::from(expr)),
                operator: (operator),
                right: (Box::from(right_expression)),
            }
        }

        expr
    }

    fn unary(&mut self) -> Expression {
        if self.match_any(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();

            let right_expression: Expression = self.unary();

            return Expression::Unary {
                operator: (operator),
                right: (Box::from(right_expression)),
            };
        }

        return self.primary();
    }

    fn primary(&mut self) -> Expression {
        if self.match_any(&[TokenType::True]) {
            return Expression::Literal {
                literal_value: (Literal::Boolean(true)),
            };
        }

        if self.match_any(&[TokenType::False]) {
            return Expression::Literal {
                literal_value: (Literal::Boolean(false)),
            };
        }

        if self.match_any(&[TokenType::Nil]) {
            return Expression::Literal {
                literal_value: (Literal::Nil),
            };
        }

        if self.match_any(&[TokenType::String, TokenType::Number]) {
            let previous_literal = self.previous().get_literal().clone();

            return Expression::Literal {
                literal_value: (previous_literal),
            };
        }

        if self.match_any(&[TokenType::LeftParen]) {
            let expression = self.expression();

            self.consume(
                &TokenType::RightParen,
                "Expect ')' after expression.".to_string(),
            );

            return Expression::Grouping {
                expression: Box::new(expression),
            };
        }

        panic!("Invalid syntax")
    }

    fn match_any(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        let current = self.peek();

        return current.type_equals_to(token_type);
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        let current = self.peek();

        current.is_eof()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token_type: &TokenType, message: String) -> &Token {
        if self.check(token_type) {
            return &self.advance();
        }
        panic!("{}", message);
    }
}
