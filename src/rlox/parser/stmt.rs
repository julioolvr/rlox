use rlox::parser::Expr;
use rlox::token::Token;

pub enum Stmt {
    Print(Expr),
    Expr(Expr),
    Var(Token, Expr),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Box<Option<Stmt>>),
    While(Expr, Box<Stmt>),
}
