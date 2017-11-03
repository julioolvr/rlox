use rlox::parser::Expr;

pub enum Stmt {
    Print(Expr),
    Expr(Expr),
}
