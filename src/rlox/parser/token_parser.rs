use rlox::token::{Token, TokenType};
use rlox::parser::errors::ParsingError;
use rlox::parser::Expr;

pub struct TokenParser {
    tokens: Vec<Token>,
    current: usize,
}

impl TokenParser {
    pub fn new(tokens: Vec<Token>) -> TokenParser {
        TokenParser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>, Vec<ParsingError>> {
        let mut statements: Vec<Expr> = Vec::new();
        let mut errors: Vec<ParsingError> = Vec::new();

        while !self.is_over() {
            match self.statement() {
                Ok(expr) => statements.push(expr),
                Err(err) => errors.push(err),
            }
        }

        if errors.len() == 0 {
            Ok(statements)
        } else {
            Err(errors)
        }
    }

    fn statement(&mut self) -> Result<Expr, ParsingError> {
        self.expression_statement()
    }

    fn expression_statement(&mut self) -> Result<Expr, ParsingError> {
        let expr = self.expression()?;

        if let Some(err) =
            self.consume(TokenType::Semicolon,
                         "Expect ';' after expression.".to_string()) {
            Err(err)
        } else {
            Ok(expr)
        }
    }

    fn expression(&mut self) -> Result<Expr, ParsingError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParsingError> {
        let mut expr = self.comparison()?;

        while self.next_is(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParsingError> {
        let mut expr = self.addition()?;

        while self.next_is(vec![TokenType::Greater,
                                TokenType::GreaterEqual,
                                TokenType::Less,
                                TokenType::LessEqual]) {
            let operator = self.previous().clone();
            let right = self.addition()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn addition(&mut self) -> Result<Expr, ParsingError> {
        let mut expr = self.multiplication()?;

        while self.next_is(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.multiplication()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn multiplication(&mut self) -> Result<Expr, ParsingError> {
        let mut expr = self.unary()?;

        while self.next_is(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParsingError> {
        if self.next_is(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParsingError> {
        if self.next_is(vec![TokenType::Number,
                             TokenType::String,
                             TokenType::False,
                             TokenType::True,
                             TokenType::Nil]) {
            return Ok(Expr::Literal(self.previous().literal.clone()));
        }

        if self.next_is(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;

            if let Some(err) =
                self.consume(TokenType::RightParen,
                             "Expected ')' after expression.".to_string()) {
                return Err(err);
            } else {
                return Ok(Expr::Grouping(Box::new(expr)));
            }
        }

        if self.is_over() {
            Err(ParsingError::UnexpectedEofError)
        } else {
            Err(ParsingError::UnexpectedTokenError(self.peek().clone(),
                                                   "Unexpected token".to_string()))
        }

    }

    // Infrastructure
    fn next_is(&mut self, types: Vec<TokenType>) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_over() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_over() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_over(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Option<ParsingError> {
        if self.check(token_type) {
            self.advance();
            None
        } else {
            Some(ParsingError::UnexpectedTokenError(self.peek().clone(), message))
        }
    }
}