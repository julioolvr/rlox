use rlox::token::{Token, TokenType};
use rlox::errors::Error;

pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner { source }
    }

    pub fn scan_tokens(&self) -> (Vec<Token>, Vec<Error>) {
        let line_number = 1;

        let mut errors: Vec<Error> = Vec::new();
        let mut tokens: Vec<Token> = self.source
            .chars()
            .fold(Vec::new(), |mut acc, c| {
                let token = match c {
                    '(' => {
                        Ok(Token::new(TokenType::LEFT_PAREN,
                                      c.to_string(),
                                      "".to_string(),
                                      line_number))
                    }
                    ')' => {
                        Ok(Token::new(TokenType::RIGHT_PAREN,
                                      c.to_string(),
                                      "".to_string(),
                                      line_number))
                    }
                    '{' => {
                        Ok(Token::new(TokenType::LEFT_BRACE,
                                      c.to_string(),
                                      "".to_string(),
                                      line_number))
                    }
                    '}' => {
                        Ok(Token::new(TokenType::RIGHT_BRACE,
                                      c.to_string(),
                                      "".to_string(),
                                      line_number))
                    }
                    ',' => {
                        Ok(Token::new(TokenType::COMMA, c.to_string(), "".to_string(), line_number))
                    }
                    '.' => {
                        Ok(Token::new(TokenType::DOT, c.to_string(), "".to_string(), line_number))
                    }
                    '-' => {
                        Ok(Token::new(TokenType::MINUS, c.to_string(), "".to_string(), line_number))
                    }
                    '+' => {
                        Ok(Token::new(TokenType::PLUS, c.to_string(), "".to_string(), line_number))
                    }
                    ';' => {
                        Ok(Token::new(TokenType::SEMICOLON,
                                      c.to_string(),
                                      "".to_string(),
                                      line_number))
                    }
                    '*' => {
                        Ok(Token::new(TokenType::STAR, c.to_string(), "".to_string(), line_number))
                    }
                    unknown_char => {
                        Err(Error::ScannerError(line_number,
                                                format!("Invalid character: {}", unknown_char)))
                    }
                };

                match token {
                    Ok(token) => acc.push(token),
                    Err(err) => errors.push(err),
                }

                acc
            });

        tokens.push(Token::new(TokenType::EOF, "".to_string(), "".to_string(), 0));

        (tokens, errors)
    }
}
