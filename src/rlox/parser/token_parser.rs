use rlox::token::{Literal, Token, TokenType};
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
        let statement = if self.next_is(vec![TokenType::Class]) {
            self.class_declaration()
        } else if self.next_is(vec![TokenType::Var]) {
            self.var_declaration()
        } else if self.next_is(vec![TokenType::Fun]) {
            self.fun_declaration("function")
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

    fn class_declaration(&mut self) -> Result<Stmt, ParsingError> {
        let name = self.consume(TokenType::Identifier, "Expected class name".to_string())?;

        self.consume(TokenType::LeftBrace,
                     format!("Expected `{{` before class body."))?;

        let mut methods = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_over() {
            methods.push(self.fun_declaration("method")?);
        }

        self.consume(TokenType::RightBrace,
                     format!("Expected `}}` after class body."))?;

        Ok(Stmt::Class(name, methods))
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

    fn fun_declaration(&mut self, kind: &'static str) -> Result<Stmt, ParsingError> {
        let name = self.consume(TokenType::Identifier, format!("Expected {} name.", kind))?;

        self.consume(TokenType::LeftParen,
                     format!("Expected `(` after {} name.", kind))?;
        let mut parameters: Vec<Token> = Vec::new();

        if !self.check(TokenType::RightParen) {
            parameters.push(self.consume(TokenType::Identifier,
                                         "Expected parameter name".to_string())?);

            while self.next_is(vec![TokenType::Comma]) {
                if parameters.len() >= 8 {
                    // TODO: The reference interpreter doesn't bail on this error,
                    // it keeps on parsing but reports it.
                    return Err(ParsingError::TooManyParametersError);
                }

                parameters.push(self.consume(TokenType::Identifier,
                                             "Expected parameter name".to_string())?)
            }
        }

        self.consume(TokenType::RightParen,
                     "Expect `)` after parameters.".to_string())?;

        self.consume(TokenType::LeftBrace,
                     format!("Expected `{{` before {} body.", kind))?;

        let body = self.block_statement()?;

        Ok(Stmt::Func(name, parameters, Box::new(body)))
    }

    fn statement(&mut self) -> Result<Stmt, ParsingError> {
        if self.next_is(vec![TokenType::Print]) {
            self.print_statement()
        } else if self.next_is(vec![TokenType::LeftBrace]) {
            self.block_statement()
        } else if self.next_is(vec![TokenType::If]) {
            self.if_statement()
        } else if self.next_is(vec![TokenType::While]) {
            self.while_statement()
        } else if self.next_is(vec![TokenType::For]) {
            self.for_statement()
        } else if self.next_is(vec![TokenType::Return]) {
            self.return_statement()
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

    fn if_statement(&mut self) -> Result<Stmt, ParsingError> {
        self.consume(TokenType::LeftParen, "Expected `(` after `if`".to_string())?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen,
                     "Expected `)` after condition".to_string())?;

        let then_branch = self.statement()?;
        let else_branch = if self.next_is(vec![TokenType::Else]) {
            Some(self.statement()?)
        } else {
            None
        };

        Ok(Stmt::If(condition, Box::new(then_branch), Box::new(else_branch)))
    }

    fn while_statement(&mut self) -> Result<Stmt, ParsingError> {
        self.consume(TokenType::LeftParen,
                     "Expected `(` after `while`".to_string())?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen,
                     "Expected `while` after condition".to_string())?;

        let body = self.statement()?;

        Ok(Stmt::While(condition, Box::new(body)))
    }

    fn for_statement(&mut self) -> Result<Stmt, ParsingError> {
        self.consume(TokenType::LeftParen, "Expected `(` after `for`".to_string())?;

        let initializer = if self.next_is(vec![TokenType::Semicolon]) {
            None
        } else if self.next_is(vec![TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if self.next_is(vec![TokenType::Semicolon]) {
            None
        } else {
            let expr = Some(self.expression()?);
            self.consume(TokenType::Semicolon,
                         "Expect `;` after loop condition.".to_string())?;
            expr
        };

        let increment = if self.next_is(vec![TokenType::RightParen]) {
            None
        } else {
            let expr = Some(self.expression()?);
            self.consume(TokenType::RightParen,
                         "Expect `)` after for clause.".to_string())?;
            expr
        };

        let mut body = self.statement()?;

        if let Some(increment_expr) = increment {
            body = Stmt::Block(vec![body, Stmt::Expr(increment_expr)])
        }

        let condition = condition.unwrap_or(Expr::Literal(Literal::Bool(true)));
        body = Stmt::While(condition, Box::new(body));

        if let Some(initializer_expr) = initializer {
            body = Stmt::Block(vec![initializer_expr, body])
        }

        Ok(body)
    }

    fn return_statement(&mut self) -> Result<Stmt, ParsingError> {
        let keyword = self.previous().clone();

        let value = if !self.check(TokenType::Semicolon) {
            self.expression()?
        } else {
            Expr::Literal(Literal::Nil)
        };

        self.consume(TokenType::Semicolon,
                     "Expect `;` after return value.".to_string())?;

        Ok(Stmt::Return(keyword, Box::new(value)))
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
        let expr = self.or()?;

        if self.next_is(vec![TokenType::Equal]) {
            let token = self.previous().clone();
            let value = self.assignment()?;

            match expr {
                Expr::Var(token, _) => {
                    return Ok(Expr::Assign(token, Box::new(value), None));
                }
                Expr::Get(target, token) => return Ok(Expr::Set(target, token, Box::new(value))),
                _ => return Err(ParsingError::InvalidAssignmentError(token)),
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, ParsingError> {
        let mut expr = self.and()?;

        while self.next_is(vec![TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParsingError> {
        let mut expr = self.equality()?;

        while self.next_is(vec![TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
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

        self.call()
    }

    fn call(&mut self) -> Result<Expr, ParsingError> {
        let mut expr = self.primary()?;

        loop {
            if self.next_is(vec![TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else if self.next_is(vec![TokenType::Dot]) {
                let name =
                    self.consume(TokenType::Identifier,
                                 "Expected property name after `.`.".to_string())?;
                expr = Expr::Get(Box::new(expr), name);
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParsingError> {
        let mut arguments: Vec<Expr> = Vec::new();

        if !self.check(TokenType::RightParen) {
            arguments.push(self.expression()?);

            while self.next_is(vec![TokenType::Comma]) {
                if arguments.len() >= 8 {
                    // TODO: The reference interpreter doesn't bail on this error,
                    // it keeps on parsing but reports it.
                    return Err(ParsingError::TooManyArgumentsError);
                }
                arguments.push(self.expression()?);
            }
        }

        let paren = self.consume(TokenType::RightParen,
                                 "Expect `)` after arguments.".to_string())?;

        Ok(Expr::Call(Box::new(callee), arguments, paren))
    }

    fn primary(&mut self) -> Result<Expr, ParsingError> {
        if self.next_is(vec![TokenType::Number,
                             TokenType::String,
                             TokenType::False,
                             TokenType::True,
                             TokenType::Nil]) {
            return match self.previous().literal {
                       Some(ref literal) => Ok(Expr::Literal(literal.clone())),
                       None => {
                           Err(ParsingError::InternalError("Missing literal value".to_string()))
                       }
                   };
        }

        if self.next_is(vec![TokenType::Identifier]) {
            return Ok(Expr::Var(self.previous().clone(), None));
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
