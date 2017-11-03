mod char_scanner;
pub mod errors;

use rlox::token::Token;
use self::errors::ScannerError;
use self::char_scanner::CharScanner;

pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner { source }
    }

    pub fn scan_tokens(&self) -> (Vec<Token>, Vec<ScannerError>) {
        let mut scanner = CharScanner::new(self.source.chars().collect());
        scanner.scan_tokens()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rlox::token::{TokenType, Literal};

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

    #[test]
    fn line_numbers() {
        let scanner = Scanner::new("123.4\nsome_ident\n\"some string\"".to_string());
        let (mut tokens, errors) = scanner.scan_tokens();

        assert_eq!(errors.len(), 0);
        tokens.pop(); // Remove the Eof token since we don't really care about its line number

        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.line, i + 1);
        }
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

            mod keywords {
                use super::*;

                test_token!(and, "and", TokenType::And);
                test_token!(class, "class", TokenType::Class);
                test_token!(else_token, "else", TokenType::Else);
                test_token!(false_token, "false", TokenType::False);
                test_token!(for_token, "for", TokenType::For);
                test_token!(fun, "fun", TokenType::Fun);
                test_token!(if_token, "if", TokenType::If);
                test_token!(nil, "nil", TokenType::Nil);
                test_token!(or, "or", TokenType::Or);
                test_token!(print, "print", TokenType::Print);
                test_token!(return_token, "return", TokenType::Return);
                test_token!(super_token, "super", TokenType::Super);
                test_token!(this, "this", TokenType::This);
                test_token!(true_token, "true", TokenType::True);
                test_token!(var, "var", TokenType::Var);
                test_token!(while_token, "while", TokenType::While);
            }
        }
    }
}