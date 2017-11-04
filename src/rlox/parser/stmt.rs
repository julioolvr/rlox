use rlox::parser::Expr;
use rlox::token::Token;

pub enum Stmt {
    Print(Expr),
    Expr(Expr),
    Var(Token, Expr),
    Block(Vec<Stmt>),
}
