use rlox::parser::Stmt;
use rlox::parser::Expr;
use rlox::token::Token;
use rlox::interpreter::Interpreter;
use std::collections::hash_map::HashMap;

pub struct Resolver {
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver { scopes: Vec::new() }
    }

    pub fn resolve_ast(&mut self, ast: &Vec<Stmt>, interpreter: &mut Interpreter) {
        for stmt in ast {
            self.resolve_statement(stmt, interpreter);
        }
    }

    fn resolve_statement(&mut self, stmt: &Stmt, interpreter: &mut Interpreter) {
        match *stmt {
            Stmt::Block(ref stmts) => {
                self.begin_scope();
                self.resolve_ast(stmts, interpreter);
                self.end_scope();
            }
            Stmt::Var(ref token, ref expr) => {
                self.declare(token.lexeme.clone());
                self.resolve_expression(expr, interpreter);

                // TODO: Can I use a reference to the string instead of having to own it?
                self.define(token.lexeme.clone());
            }
            Stmt::Func(ref token, ref params, ref body) => {
                self.declare(token.lexeme.clone());
                self.define(token.lexeme.clone());

                self.resolve_function(params, body, interpreter);
            }
            Stmt::Expr(ref expr) => self.resolve_expression(expr, interpreter),
            Stmt::If(ref condition, ref then_branch, ref else_branch) => {
                self.resolve_expression(condition, interpreter);
                self.resolve_statement(then_branch, interpreter);

                if let Some(ref else_branch) = **else_branch {
                    self.resolve_statement(else_branch, interpreter);
                }
            }
            Stmt::Print(ref expr) => self.resolve_expression(expr, interpreter),
            Stmt::Return(_, ref expr) => self.resolve_expression(expr, interpreter),
            Stmt::While(ref condition, ref body) => {
                self.resolve_expression(condition, interpreter);
                self.resolve_statement(body, interpreter);
            }
        }
    }

    fn resolve_expression(&mut self, expr: &Expr, interpreter: &mut Interpreter) {
        match *expr {
            Expr::Var(ref token) => {
                if let Some(scope) = self.scopes.last() {
                    if let Some(is_var_available) = scope.get(&token.lexeme) {
                        if !is_var_available {
                            // TODO: Error
                        }
                    }
                }

                self.resolve_local(expr, token, interpreter);
            }
            Expr::Assign(ref token, ref expr) => {
                self.resolve_expression(expr, interpreter);
                self.resolve_local(expr, token, interpreter);
            }
            Expr::Binary(ref left, _, ref right) => {
                self.resolve_expression(left, interpreter);
                self.resolve_expression(right, interpreter);
            }
            Expr::Call(ref callee, ref arguments, _) => {
                self.resolve_expression(callee, interpreter);

                for arg in arguments {
                    self.resolve_expression(arg, interpreter);
                }
            }
            Expr::Grouping(ref expr) => {
                self.resolve_expression(expr, interpreter);
            }
            Expr::Literal(_) => {}
            Expr::Logical(ref left, _, ref right) => {
                self.resolve_expression(left, interpreter);
                self.resolve_expression(right, interpreter);
            }
            Expr::Unary(_, ref expr) => {
                self.resolve_expression(expr, interpreter);
            }
        }
    }

    fn resolve_local(&self, expr: &Expr, token: &Token, interpreter: &mut Interpreter) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&token.lexeme) {
                interpreter.resolve(expr, i);
                break;
            }
        }
    }

    fn resolve_function(&mut self,
                        params: &Vec<Token>,
                        body: &Stmt,
                        interpreter: &mut Interpreter) {
        self.begin_scope();

        for param in params {
            self.declare(param.lexeme.clone());
            self.define(param.lexeme.clone());
        }

        self.resolve_statement(body, interpreter);
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