use rlox::parser::Expr;
use rlox::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Expr),
    Expr(Expr),
    Var(Token, Expr),
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>, Box<Option<Stmt>>),
    While(Expr, Box<Stmt>),
    Func(Token, Vec<Token>, Box<Stmt>),
    Return(Token, Box<Expr>),
    Class(Token, Vec<Stmt>),
}
