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
        let mut scanner = CharScanner::new(self.source.chars().collect());
        scanner.scan_tokens()
    }
}

struct CharScanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

impl CharScanner {
    pub fn new(source: Vec<char>) -> CharScanner {
        CharScanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> (Vec<Token>, Vec<Error>) {
        let mut errors: Vec<Error> = Vec::new();
        let mut tokens: Vec<Token> = Vec::new();

        while !self.is_eof() {
            self.start = self.current;

            self.advance();
            let token = self.scan_token();

            match token {
                Ok(token) => {
                    if let Some(token) = token {
                        tokens.push(token)
                    }
                }
                Err(err) => errors.push(err),
            }
        }

        tokens.push(Token::new(TokenType::EOF, "".to_string(), "".to_string(), 0));
        (tokens, errors)
    }

    fn scan_token(&mut self) -> Result<Option<Token>, Error> {
        let ch = self.source[self.current - 1];

        match ch {
            '(' => Ok(self.build_current_token(TokenType::LEFT_PAREN)),
            ')' => Ok(self.build_current_token(TokenType::RIGHT_PAREN)),
            '{' => Ok(self.build_current_token(TokenType::LEFT_BRACE)),
            '}' => Ok(self.build_current_token(TokenType::RIGHT_BRACE)),
            ',' => Ok(self.build_current_token(TokenType::COMMA)),
            '.' => Ok(self.build_current_token(TokenType::DOT)),
            '-' => Ok(self.build_current_token(TokenType::MINUS)),
            '+' => Ok(self.build_current_token(TokenType::PLUS)),
            ';' => Ok(self.build_current_token(TokenType::SEMICOLON)),
            '*' => Ok(self.build_current_token(TokenType::STAR)),
            '!' => {
                let token_type = if self.is_match('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };

                Ok(self.build_current_token(token_type))
            }
            '=' => {
                let token_type = if self.is_match('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };

                Ok(self.build_current_token(token_type))
            }
            '<' => {
                let token_type = if self.is_match('=') {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                };

                Ok(self.build_current_token(token_type))
            }
            '>' => {
                let token_type = if self.is_match('=') {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                };

                Ok(self.build_current_token(token_type))
            }
            '/' => {
                if self.is_match('/') {
                    while self.peek() != '\n' && !self.is_eof() {
                        self.advance();
                    }

                    Ok(None)
                } else {
                    Ok(self.build_current_token(TokenType::SLASH))
                }
            }
            unknown_char => {
                Err(Error::ScannerError(self.line, format!("Invalid character: {}", unknown_char)))
            }
        }
    }

    fn is_match(&mut self, c: char) -> bool {
        if self.is_eof() {
            return false;
        }

        if self.source[self.current] != c {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_eof() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn current_lexeme(&self) -> String {
        self.source[self.start..self.current]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    fn is_eof(&self) -> bool {
        self.current >= self.source.len()
    }

    fn build_current_token(&self, token_type: TokenType) -> Option<Token> {
        Some(Token::new(token_type, self.current_lexeme(), "".to_string(), self.line))
    }
}
