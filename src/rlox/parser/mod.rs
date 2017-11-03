mod token_parser;
pub mod expr;

use rlox::token::Token;
use rlox::errors::Error;
use self::expr::Expr;
use self::token_parser::TokenParser;

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens }
    }

    pub fn ast(&self) -> Result<Expr, Error> {
        TokenParser::new(self.tokens.clone()).expression()
    }
}