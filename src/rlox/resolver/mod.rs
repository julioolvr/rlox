use rlox::parser::Stmt;
use rlox::parser::Expr;
use rlox::token::Token;
use std::collections::hash_map::HashMap;

pub struct Resolver {
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver { scopes: Vec::new() }
    }

    pub fn resolve_ast(&mut self, ast: &mut Vec<Stmt>) {
        for ref mut stmt in ast {
            self.resolve_statement(stmt);
        }
    }

    fn resolve_statement(&mut self, stmt: &mut Stmt) {
        match *stmt {
            Stmt::Block(ref mut stmts) => {
                self.begin_scope();
                self.resolve_ast(stmts);
                self.end_scope();
            }
            Stmt::Var(ref token, ref mut expr) => {
                self.declare(token.lexeme.clone());
                self.resolve_expression(expr);

                // TODO: Can I use a reference to the string instead of having to own it?
                self.define(token.lexeme.clone());
            }
            Stmt::Func(ref token, ref params, ref mut body) => {
                self.declare(token.lexeme.clone());
                self.define(token.lexeme.clone());

                self.resolve_function(params, body);
            }
            Stmt::Expr(ref mut expr) => self.resolve_expression(expr),
            Stmt::If(ref mut condition, ref mut then_branch, ref mut else_branch) => {
                self.resolve_expression(condition);
                self.resolve_statement(then_branch);

                if let Some(ref mut else_branch) = **else_branch {
                    self.resolve_statement(else_branch);
                }
            }
            Stmt::Print(ref mut expr) => self.resolve_expression(expr),
            Stmt::Return(_, ref mut expr) => self.resolve_expression(expr),
            Stmt::While(ref mut condition, ref mut body) => {
                self.resolve_expression(condition);
                self.resolve_statement(body);
            }
            Stmt::Class(ref token, _) => {
                self.declare(token.lexeme.clone());
                self.define(token.lexeme.clone());
            }
        }
    }

    fn resolve_expression(&mut self, expr: &mut Expr) {
        match *expr {
            Expr::Var(ref token, ref mut distance) => {
                if let Some(scope) = self.scopes.last() {
                    if let Some(is_var_available) = scope.get(&token.lexeme) {
                        if !is_var_available {
                            // TODO: Error
                        }
                    }
                }

                *distance = self.resolve_local(token.lexeme.clone());
            }
            Expr::Assign(ref token, ref mut expr, ref mut distance) => {
                self.resolve_expression(expr);
                *distance = self.resolve_local(token.lexeme.clone());
            }
            Expr::Binary(ref mut left, _, ref mut right) => {
                self.resolve_expression(left);
                self.resolve_expression(right);
            }
            Expr::Call(ref mut callee, ref mut arguments, _) => {
                self.resolve_expression(callee);

                for ref mut arg in arguments {
                    self.resolve_expression(arg);
                }
            }
            Expr::Grouping(ref mut expr) => {
                self.resolve_expression(expr);
            }
            Expr::Literal(_) => {}
            Expr::Logical(ref mut left, _, ref mut right) => {
                self.resolve_expression(left);
                self.resolve_expression(right);
            }
            Expr::Unary(_, ref mut expr) => {
                self.resolve_expression(expr);
            }
        }
    }

    fn resolve_local(&self, lexeme: String) -> Option<usize> {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&lexeme) {
                return Some(i);
            }
        }

        None
    }

    fn resolve_function(&mut self, params: &Vec<Token>, body: &mut Stmt) {
        self.begin_scope();

        for param in params {
            self.declare(param.lexeme.clone());
            self.define(param.lexeme.clone());
        }

        self.resolve_statement(body);
        self.end_scope();
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: String) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, false);
        }
    }

    fn define(&mut self, name: String) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, true);
        }
    }
}
