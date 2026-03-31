use std::collections::HashMap;
use crate::parser::{CmpOp, Condition, Expr, FieldDef, MethodDef, Param, Stmt, TypeAnnotation};

// ── Values ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Str(String),
    /// An instance of a struct
    Instance {
        struct_name: String,
        fields: HashMap<String, Value>,
    },
    Nil,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => {
                if n.fract() == 0.0 { write!(f, "{}", *n as i64) } else { write!(f, "{}", n) }
            }
            Value::Str(s) => write!(f, "{}", s),
            Value::Instance { struct_name, .. } => write!(f, "<{} instance>", struct_name),
            Value::Nil => write!(f, "nil"),
        }
    }
}

// ── Struct registry ──────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct StructDef {
    pub fields: Vec<FieldDef>,
    pub methods: Vec<MethodDef>,
}

// ── Signals ──────────────────────────────────────────────────────────────────

#[derive(Debug)]
enum Signal {
    None,
    Break,
    Continue,
    Return(Value),
}

// ── Interpreter ──────────────────────────────────────────────────────────────

pub struct Interpreter {
    /// Global / local variable scope (flat for now)
    vars: HashMap<String, Value>,
    /// Registered struct definitions
    structs: HashMap<String, StructDef>,
    /// Registered top-level functions: name -> (params, body)
    functions: HashMap<String, (Vec<Param>, Vec<Stmt>)>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            vars: HashMap::new(),
            structs: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    // ── Expressions ──────────────────────────────────────────────────────────

    fn eval_expr(&self, expr: &Expr) -> Value {
        match expr {
            Expr::StringLiteral(s) => Value::Str(s.clone()),
            Expr::NumberLiteral(n) => Value::Number(*n),
            Expr::Identifier(name) => {
                self.vars.get(name).cloned().unwrap_or_else(|| {
                    eprintln!("Runtime error: undefined variable '{}'", name);
                    std::process::exit(1);
                })
            }
            Expr::FieldAccess { object, field } => {
                let obj = self.eval_expr(object);
                match obj {
                    Value::Instance { fields, .. } => {
                        fields.get(field).cloned().unwrap_or_else(|| {
                            eprintln!("Runtime error: unknown field '{}'", field);
                            std::process::exit(1);
                        })
                    }
                    _ => { eprintln!("Runtime error: field access on non-instance"); std::process::exit(1); }
                }
            }
            Expr::Call { callee, args } => {
                self.eval_call(callee, args)
            }
        }
    }

    fn eval_call(&self, callee: &Expr, args: &[Expr]) -> Value {
        let arg_vals: Vec<Value> = args.iter().map(|a| self.eval_expr(a)).collect();

        match callee {
            // plain function call: greet(...)
            Expr::Identifier(name) => {
                // built-in: new StructName(...)  -- create instance
                if let Some(def) = self.structs.get(name) {
                    return self.create_instance(name, def, &arg_vals);
                }
                // user-defined function
                if let Some((params, body)) = self.functions.get(name) {
                    return self.call_function(params.clone(), body.clone(), &arg_vals, None);
                }
                eprintln!("Runtime error: undefined function or struct '{}'", name);
                std::process::exit(1);
            }
            // method call: instance.Method(...)
            Expr::FieldAccess { object, field } => {
                let obj = self.eval_expr(object);
                match &obj {
                    Value::Instance { struct_name, fields: _ } => {
                        let def = self.structs.get(struct_name).cloned().unwrap_or_else(|| {
                            eprintln!("Runtime error: unknown struct '{}'", struct_name);
                            std::process::exit(1);
                        });
                        let method = def.methods.iter().find(|m| &m.name == field).cloned()
                            .unwrap_or_else(|| {
                                eprintln!("Runtime error: unknown method '{}' on '{}'", field, struct_name);
                                std::process::exit(1);
                            });
                        return self.call_function(
                            method.params.clone(),
                            method.body.clone(),
                            &arg_vals,
                            Some(obj.clone()),
                        );
                    }
                    _ => { eprintln!("Runtime error: method call on non-instance"); std::process::exit(1); }
                }
            }
            _ => { eprintln!("Runtime error: not callable"); std::process::exit(1); }
        }
    }

    /// Create a struct instance with default field values
    fn create_instance(&self, name: &str, def: &StructDef, _args: &[Value]) -> Value {
        let mut fields = HashMap::new();
        for f in &def.fields {
            let default = match &f.ty {
                TypeAnnotation::Inombolo   => Value::Number(0.0),
                TypeAnnotation::Ibala      => Value::Str(String::new()),
                TypeAnnotation::Named(_)   => Value::Nil,
            };
            fields.insert(f.name.clone(), default);
        }
        Value::Instance { struct_name: name.to_string(), fields }
    }

    /// Execute a function/method body in a child scope
    fn call_function(
        &self,
        params: Vec<Param>,
        body: Vec<Stmt>,
        args: &[Value],
        self_val: Option<Value>,
    ) -> Value {
        // build child interpreter with its own var scope
        let mut child = Interpreter {
            vars: self.vars.clone(),
            structs: self.structs.clone(),
            functions: self.functions.clone(),
        };

        // bind _lona if this is a method call
        if let Some(sv) = self_val {
            child.vars.insert("mina".to_string(), sv);
        }

        // bind parameters
        for (i, param) in params.iter().enumerate() {
            let val = args.get(i).cloned().unwrap_or(Value::Nil);
            child.vars.insert(param.name.clone(), val);
        }

        match child.exec_block(&body) {
            Signal::Return(v) => v,
            _ => Value::Nil,
        }
    }

    // ── Conditions ───────────────────────────────────────────────────────────

    fn eval_condition(&self, cond: &Condition) -> bool {
        let l = self.eval_expr(&cond.left);
        let r = self.eval_expr(&cond.right);
        match (&cond.op, &l, &r) {
            (CmpOp::EqEq,  Value::Number(a), Value::Number(b)) => a == b,
            (CmpOp::NotEq, Value::Number(a), Value::Number(b)) => a != b,
            (CmpOp::Lt,    Value::Number(a), Value::Number(b)) => a < b,
            (CmpOp::Gt,    Value::Number(a), Value::Number(b)) => a > b,
            (CmpOp::LtEq,  Value::Number(a), Value::Number(b)) => a <= b,
            (CmpOp::GtEq,  Value::Number(a), Value::Number(b)) => a >= b,
            (CmpOp::EqEq,  Value::Str(a),    Value::Str(b))    => a == b,
            (CmpOp::NotEq, Value::Str(a),    Value::Str(b))    => a != b,
            _ => { eprintln!("Runtime error: cannot compare {:?} and {:?}", l, r); std::process::exit(1); }
        }
    }

    // ── Execution ─────────────────────────────────────────────────────────────

    fn exec_block(&mut self, stmts: &[Stmt]) -> Signal {
        for stmt in stmts {
            match self.exec_stmt(stmt) {
                Signal::None => {}
                sig => return sig,
            }
        }
        Signal::None
    }

    fn exec_stmt(&mut self, stmt: &Stmt) -> Signal {
        match stmt {
            Stmt::DeclareNumber { name, value } => {
                self.vars.insert(name.clone(), Value::Number(*value));
            }
            Stmt::DeclareString { name, value } => {
                self.vars.insert(name.clone(), Value::Str(value.clone()));
            }
            Stmt::Print(expr) => {
                println!("{}", self.eval_expr(expr));
            }
            Stmt::If { condition, body } => {
                if self.eval_condition(condition) {
                    return self.exec_block(body);
                }
            }
            Stmt::While { condition, body } => {
                loop {
                    if !self.eval_condition(condition) { break; }
                    match self.exec_block(body) {
                        Signal::Break    => break,
                        Signal::Continue => continue,
                        sig @ Signal::Return(_) => return sig,
                        Signal::None     => {}
                    }
                }
            }
            Stmt::Loop { body } => {
                loop {
                    match self.exec_block(body) {
                        Signal::Break    => break,
                        Signal::Continue => continue,
                        sig @ Signal::Return(_) => return sig,
                        Signal::None     => {}
                    }
                }
            }
            Stmt::Break    => return Signal::Break,
            Stmt::Continue => return Signal::Continue,
            Stmt::Return(expr) => {
                let val = expr.as_ref().map(|e| self.eval_expr(e)).unwrap_or(Value::Nil);
                return Signal::Return(val);
            }
            Stmt::FunctionDef { name, params, body } => {
                self.functions.insert(name.clone(), (params.clone(), body.clone()));
            }
            Stmt::StructDef { name, fields, methods } => {
                self.structs.insert(name.clone(), StructDef {
                    fields: fields.clone(),
                    methods: methods.clone(),
                });
            }
            Stmt::Instantiate { struct_name, var_name } => {
                let def = self.structs.get(struct_name).cloned().unwrap_or_else(|| {
                    eprintln!("Runtime error: unknown struct '{}'", struct_name);
                    std::process::exit(1);
                });
                let instance = self.create_instance(struct_name, &def, &[]);
                self.vars.insert(var_name.clone(), instance);
            }
            Stmt::ExprStmt(expr) => {
                self.eval_expr(expr);
            }
            // mina.field = value;  -- mutate the mina instance in scope
            Stmt::FieldAssign { field, value } => {
                let val = self.eval_expr(value);
                if let Some(Value::Instance { fields, .. }) = self.vars.get_mut("mina") {
                    fields.insert(field.clone(), val);
                } else {
                    eprintln!("Runtime error: mina.{} used outside a method", field);
                    std::process::exit(1);
                }
            }
        }
        Signal::None
    }

    pub fn run(&mut self, stmts: &[Stmt]) {
        self.exec_block(stmts);
    }
}
