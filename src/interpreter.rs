use crate::parser::{Expr, Stmt};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    fn eval_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::StringLiteral(s) => s.clone(),
        }
    }

    pub fn run(&self, stmts: &[Stmt]) {
        for stmt in stmts {
            match stmt {
                Stmt::Print(expr) => {
                    let value = self.eval_expr(expr);
                    println!("{}", value);
                }
            }
        }
    }
}
