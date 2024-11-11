use crate::scanner::*;

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

impl Expression {
    pub fn to_string(&self) -> String {
        match self {
            Expression::Grouping { expression } => format!("(group {})", expression.to_string()),
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                format!(
                    "({} {} {})",
                    operator.lexeme(),
                    left.to_string(),
                    right.to_string()
                )
            }
            Expression::Unary { operator, right } => {
                format!("({} {})", operator.lexeme(), right.to_string())
            }
            Expression::Literal { literal_value } => format!("{}", literal_value.to_string()),
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

    // expression     → ...
    // equality       → ...
    // comparison     → ...
    // term           → ...
    // factor         → ...
    // unary          → ...
    // primary        → ...
    pub fn expression(&mut self) -> Expression {
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

    fn get_literal(&self) -> &Literal {
        let current = self.peek();

        current.get_literal()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
