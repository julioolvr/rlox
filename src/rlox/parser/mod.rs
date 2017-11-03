mod token_parser;
pub mod errors;
pub mod expr;

use rlox::token::Token;
use self::errors::ParsingError;
use self::expr::Expr;
use self::token_parser::TokenParser;

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens }
    }

    pub fn ast(&self) -> Result<Vec<Expr>, Vec<ParsingError>> {
        TokenParser::new(self.tokens.clone()).parse()
    }
}