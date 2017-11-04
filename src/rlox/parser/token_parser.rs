use rlox::token::{Token, TokenType, Literal};
use rlox::parser::errors::ParsingError;
use rlox::parser::{Expr, Stmt};

pub struct TokenParser {
    tokens: Vec<Token>,
    current: usize,
}

impl TokenParser {
    pub fn new(tokens: Vec<Token>) -> TokenParser {
        TokenParser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Vec<ParsingError>> {
        let mut statements: Vec<Stmt> = Vec::new();
        let mut errors: Vec<ParsingError> = Vec::new();

        while !self.is_over() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => errors.push(err),
            }
        }

        if errors.len() == 0 {
            Ok(statements)
        } else {
            Err(errors)
        }
    }

    fn declaration(&mut self) -> Result<Stmt, ParsingError> {
        let statement = if self.next_is(vec![TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };

        match statement {
            Ok(stmt) => Ok(stmt),
            Err(err) => {
                self.synchronize();
                Err(err)
            }
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParsingError> {
        let name = self.consume(TokenType::Identifier, "Expected variable name".to_string())?;

        let initial_value = if self.next_is(vec![TokenType::Equal]) {
            self.expression()?
        } else {
            Expr::Literal(Literal::Nil)
        };

        self.consume(TokenType::Semicolon,
                     "Expect ';' after variable declaration.".to_string())?;
        Ok(Stmt::Var(name, initial_value))
    }

    fn statement(&mut self) -> Result<Stmt, ParsingError> {
        if self.next_is(vec![TokenType::Print]) {
            self.print_statement()
        } else if self.next_is(vec![TokenType::LeftBrace]) {
            self.block_statement()
        } else {
            self.expression_statement()
        }
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParsingError> {
        let expr = self.expression()?;

        match self.consume(TokenType::Semicolon,
                           "Expect ';' after expression.".to_string()) {
            Ok(_) => Ok(Stmt::Expr(expr)),
            Err(err) => Err(err),
        }
    }

    fn block_statement(&mut self) -> Result<Stmt, ParsingError> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_over() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace,
                     "Expected `}` after block".to_string())?;

        Ok(Stmt::Block(statements))
    }

    fn print_statement(&mut self) -> Result<Stmt, ParsingError> {
        let expr = self.expression()?;

        match self.consume(TokenType::Semicolon,
                           "Expect ';' after expression.".to_string()) {
            Ok(_) => Ok(Stmt::Print(expr)),
            Err(err) => Err(err),
        }
    }

    fn expression(&mut self) -> Result<Expr, ParsingError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParsingError> {
        let expr = self.equality()?;

        if self.next_is(vec![TokenType::Equal]) {
            let token = self.previous().clone();
            let value = self.assignment()?;

            match expr {
                Expr::Var(token) => {
                    return Ok(Expr::Assign(token, Box::new(value)));
                }
                _ => return Err(ParsingError::InvalidAssignmentError(token)),
            }
        }

        Ok(expr)
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

        if self.next_is(vec![TokenType::Identifier]) {
            return Ok(Expr::Var(self.previous().clone()));
        }

        if self.next_is(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;

            match self.consume(TokenType::RightParen,
                               "Expected ')' after expression.".to_string()) {
                Ok(_) => return Ok(Expr::Grouping(Box::new(expr))),
                Err(err) => return Err(err),
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

    fn consume(&mut self, token_type: TokenType, message: String) -> Result<Token, ParsingError> {
        if self.check(token_type) {
            Ok(self.advance().clone())
        } else {
            Err(ParsingError::UnexpectedTokenError(self.peek().clone(), message))
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_over() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class | TokenType::Fun | TokenType::Var | TokenType::For |
                TokenType::If | TokenType::While | TokenType::Print | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }
}