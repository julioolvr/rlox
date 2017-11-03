mod token_parser;
mod stmt;
pub mod errors;
pub mod expr;

use rlox::token::Token;
use self::errors::ParsingError;
use self::token_parser::TokenParser;
pub use self::expr::Expr;
pub use self::stmt::Stmt;

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens }
    }

    pub fn ast(&self) -> Result<Vec<Stmt>, Vec<ParsingError>> {
        TokenParser::new(self.tokens.clone()).parse()
    }
}