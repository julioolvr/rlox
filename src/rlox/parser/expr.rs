use std;
use rlox::token::{Token, Literal};

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(Token, Box<Expr>),
    Var(Token),
    Assign(Token, Box<Expr>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Expr::Binary(ref left, ref operator, ref right) => {
                write!(f, "({} {} {})", operator.lexeme, left, right)
            }
            Expr::Grouping(ref expr) => write!(f, "(group {})", expr),
            Expr::Literal(ref literal) => write!(f, "{}", literal),
            Expr::Unary(ref operator, ref expr) => write!(f, "({} {})", operator.lexeme, expr),
            Expr::Var(ref token) => write!(f, "(var {})", token.lexeme),
            Expr::Assign(ref token, ref expr) => write!(f, "(assign {} {})", token.lexeme, expr),
        }
    }
}
