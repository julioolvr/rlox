use rlox::token::{Token, TokenType, Literal, KEYWORDS};
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

        tokens.push(Token::new(TokenType::Eof, "".to_string(), Literal::None, self.line));
        (tokens, errors)
    }

    fn scan_token(&mut self) -> Result<Option<Token>, Error> {
        let ch = self.source[self.current - 1];

        match ch {
            '(' => Ok(self.build_current_token(TokenType::LeftParen)),
            ')' => Ok(self.build_current_token(TokenType::RightParen)),
            '{' => Ok(self.build_current_token(TokenType::LeftBrace)),
            '}' => Ok(self.build_current_token(TokenType::RightBrace)),
            ',' => Ok(self.build_current_token(TokenType::Comma)),
            '.' => Ok(self.build_current_token(TokenType::Dot)),
            '-' => Ok(self.build_current_token(TokenType::Minus)),
            '+' => Ok(self.build_current_token(TokenType::Plus)),
            ';' => Ok(self.build_current_token(TokenType::Semicolon)),
            '*' => Ok(self.build_current_token(TokenType::Star)),
            '!' => {
                let token_type = if self.is_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };

                Ok(self.build_current_token(token_type))
            }
            '=' => {
                let token_type = if self.is_match('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };

                Ok(self.build_current_token(token_type))
            }
            '<' => {
                let token_type = if self.is_match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };

                Ok(self.build_current_token(token_type))
            }
            '>' => {
                let token_type = if self.is_match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
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
                    Ok(self.build_current_token(TokenType::Slash))
                }
            }
            ' ' | '\r' | '\t' => Ok(None),
            '\n' => {
                self.line += 1;
                Ok(None)
            }
            '"' => self.scan_string_literal(),
            '0'...'9' => self.scan_numeric_literal(),
            'a'...'z' | 'A'...'Z' | '_' => self.scan_identifier(),
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

    fn peek_next(&self) -> char {
        if self.current >= self.source.len() + 1 {
            '\0'
        } else {
            self.source[self.current + 1]
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
        self.build_token_with_literal(token_type, Literal::None)
    }

    fn build_token_with_literal(&self, token_type: TokenType, literal: Literal) -> Option<Token> {
        Some(Token::new(token_type, self.current_lexeme(), literal, self.line))
    }

    fn scan_string_literal(&mut self) -> Result<Option<Token>, Error> {
        while self.peek() != '"' && !self.is_eof() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_eof() {
            return Err(Error::ScannerError(self.line, "Unterminated string".to_string()));
        }

        // Once more to cover the closing "
        self.advance();

        let literal = self.source[self.start + 1..self.current - 1]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("");

        Ok(self.build_token_with_literal(TokenType::String, Literal::String(literal)))
    }

    fn scan_numeric_literal(&mut self) -> Result<Option<Token>, Error> {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            // Consume the .
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let literal = self.current_lexeme().parse::<f64>().unwrap();
        Ok(self.build_token_with_literal(TokenType::Number, Literal::Number(literal)))
    }

    fn scan_identifier(&mut self) -> Result<Option<Token>, Error> {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let token_type = KEYWORDS
            .get(self.current_lexeme().as_str())
            .map_or(TokenType::Identifier, |&token_type| token_type);

        Ok(self.build_current_token(token_type))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_only_eof_token_for_empty_source() {
        let scanner = Scanner::new("".to_string());
        let (tokens, errors) = scanner.scan_tokens();

        assert_eq!(errors.len(), 0);
        assert_eq!(tokens.len(), 1);
        let token = tokens.get(0).unwrap();
        assert_eq!(token.token_type, TokenType::Eof);
    }

    #[test]
    fn error_on_unknown_character() {
        let scanner = Scanner::new("%".to_string());
        let (tokens, errors) = scanner.scan_tokens();
        assert_eq!(tokens.len(), 1); // Eof token
        assert_eq!(errors.len(), 1);
    }

    mod tokens {
        use super::*;

        macro_rules! test_token {
            ($name:ident, $code:expr, $token_type:expr) => {
                #[test]
                fn $name() {
                    let scanner = Scanner::new($code.to_string());
                    let (tokens, errors) = scanner.scan_tokens();

                    assert_eq!(errors.len(), 0);
                    let token = tokens.get(0).unwrap();

                    assert_eq!(token.token_type, $token_type);
                }
            }
        }

        macro_rules! test_no_token {
            ($name:ident, $code:expr) => {
                #[test]
                fn $name() {
                    let scanner = Scanner::new($code.to_string());
                    let (tokens, errors) = scanner.scan_tokens();

                    assert_eq!(errors.len(), 0);
                    assert_eq!(tokens.len(), 1); // Single eof token
                }
            }
        }

        test_token!(left_paren, "(", TokenType::LeftParen);
        test_token!(right_paren, ")", TokenType::RightParen);
        test_token!(left_brace, "{", TokenType::LeftBrace);
        test_token!(right_brace, "}", TokenType::RightBrace);
        test_token!(comma, ",", TokenType::Comma);
        test_token!(dot, ".", TokenType::Dot);
        test_token!(minus, "-", TokenType::Minus);
        test_token!(plus, "+", TokenType::Plus);
        test_token!(semicolon, ";", TokenType::Semicolon);
        test_token!(star, "*", TokenType::Star);
        test_token!(bang, "!", TokenType::Bang);
        test_token!(bang_equal, "!=", TokenType::BangEqual);
        test_token!(equal, "=", TokenType::Equal);
        test_token!(equal_equal, "==", TokenType::EqualEqual);
        test_token!(less, "<", TokenType::Less);
        test_token!(less_equal, "<=", TokenType::LessEqual);
        test_token!(greater, ">", TokenType::Greater);
        test_token!(greater_equal, ">=", TokenType::GreaterEqual);
        test_token!(slash, "/", TokenType::Slash);
        test_no_token!(comments, "// some comment");
        test_no_token!(white_space, "\n\r \t");

        mod literals {
            use super::*;

            #[test]
            fn string() {
                let scanner = Scanner::new("\"some string\"".to_string());
                let (tokens, errors) = scanner.scan_tokens();

                assert_eq!(errors.len(), 0);
                let token = tokens.get(0).unwrap();

                assert_eq!(token.token_type, TokenType::String);

                match token.literal {
                    Literal::String(ref value) => assert_eq!(value, "some string"),
                    _ => assert!(false, "Should be a Literal::String"),
                }
            }

            #[test]
            fn number() {
                let scanner = Scanner::new("123.45".to_string());
                let (tokens, errors) = scanner.scan_tokens();

                assert_eq!(errors.len(), 0);
                let token = tokens.get(0).unwrap();

                assert_eq!(token.token_type, TokenType::Number);

                match token.literal {
                    Literal::Number(value) => assert_eq!(value, 123.45),
                    _ => assert!(false, "Should be a Literal::Number"),
                }
            }

            #[test]
            fn identifier() {
                let scanner = Scanner::new("my_var".to_string());
                let (tokens, errors) = scanner.scan_tokens();

                assert_eq!(errors.len(), 0);
                let token = tokens.get(0).unwrap();

                assert_eq!(token.token_type, TokenType::Identifier);
                assert_eq!(token.lexeme, "my_var");
            }
        }
    }
}