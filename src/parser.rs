use crate::lexer::Token;

// ── Types ────────────────────────────────────────────────────────────────────

/// The type annotations used in field/param declarations
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum TypeAnnotation {
    Inombolo,        // float64
    Ibala,           // string
    Named(String),   // a struct type
}

// ── Expressions ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Expr {
    StringLiteral(String),
    NumberLiteral(f64),
    /// A plain variable or struct-instance name
    Identifier(String),
    /// _lona.field
    FieldAccess { object: Box<Expr>, field: String },
    /// fn_name(args)  or  instance.method(args)
    Call { callee: Box<Expr>, args: Vec<Expr> },
}

// ── Conditions ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum CmpOp { EqEq, NotEq, Lt, Gt, LtEq, GtEq }

#[derive(Debug, Clone)]
pub struct Condition {
    pub left: Expr,
    pub op: CmpOp,
    pub right: Expr,
}

// ── Statements ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Param {
    pub name: String,
    pub ty: TypeAnnotation,
}

#[derive(Debug, Clone)]
pub struct FieldDef {
    pub name: String,
    pub ty: TypeAnnotation,
}

#[derive(Debug, Clone)]
pub struct MethodDef {
    pub name: String,
    pub params: Vec<Param>,   // does NOT include _lona, that's implicit
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    /// inombolo x = 4.6;
    DeclareNumber { name: String, value: f64 },
    /// ibala s = "hi";
    DeclareString { name: String, value: String },
    /// bhala(expr);
    Print(Expr),
    /// uma (cond) { body }
    If { condition: Condition, body: Vec<Stmt> },
    /// phindaUma (cond) { body }
    While { condition: Condition, body: Vec<Stmt> },
    /// phinda { body }
    Loop { body: Vec<Stmt> },
    /// phuma;
    Break,
    /// qhubeka;
    Continue,
    /// buya expr;
    Return(Option<Expr>),
    /// isigoqelo name(params) { body }
    FunctionDef { name: String, params: Vec<Param>, body: Vec<Stmt> },
    /// isakhi Name { fields... methods... }
    StructDef { name: String, fields: Vec<FieldDef>, methods: Vec<MethodDef> },
    /// umba Server s;
    Instantiate { struct_name: String, var_name: String },
    /// expr;  -- covers calls like server.Start(); or greet("hi");
    ExprStmt(Expr),
    /// mina.field = expr;
    FieldAssign { field: String, value: Expr },
    /// varName.field = expr;
    InstanceFieldAssign { var_name: String, field: String, value: Expr },
}

// ── Parser ───────────────────────────────────────────────────────────────────

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token { &self.tokens[self.pos] }

    fn advance(&mut self) -> Token {
        let tok = self.tokens[self.pos].clone();
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: &Token) {
        let tok = self.advance();
        if &tok != expected {
            eprintln!("Syntax error: expected {:?} but got {:?}", expected, tok);
            std::process::exit(1);
        }
    }

    fn expect_identifier(&mut self) -> String {
        match self.advance() {
            Token::Identifier(name) => name,
            tok => { eprintln!("Syntax error: expected identifier, got {:?}", tok); std::process::exit(1); }
        }
    }

    fn parse_type(&mut self) -> TypeAnnotation {
        match self.advance() {
            Token::Inombolo          => TypeAnnotation::Inombolo,
            Token::Ibala             => TypeAnnotation::Ibala,
            Token::Identifier(name)  => TypeAnnotation::Named(name),
            tok => { eprintln!("Syntax error: expected a type, got {:?}", tok); std::process::exit(1); }
        }
    }

    /// Parse comma-separated params:  name: type, name: type
    fn parse_params(&mut self) -> Vec<Param> {
        let mut params = Vec::new();
        self.expect(&Token::LParen);
        while self.peek() != &Token::RParen && self.peek() != &Token::EOF {
            let name = self.expect_identifier();
            self.expect(&Token::Colon);
            let ty = self.parse_type();
            params.push(Param { name, ty });
            if self.peek() == &Token::Comma { self.advance(); }
        }
        self.expect(&Token::RParen);
        params
    }

    /// Parse comma-separated call arguments
    fn parse_args(&mut self) -> Vec<Expr> {
        let mut args = Vec::new();
        self.expect(&Token::LParen);
        while self.peek() != &Token::RParen && self.peek() != &Token::EOF {
            args.push(self.parse_expr());
            if self.peek() == &Token::Comma { self.advance(); }
        }
        self.expect(&Token::RParen);
        args
    }

    fn parse_cmp_op(&mut self) -> CmpOp {
        match self.advance() {
            Token::EqEq  => CmpOp::EqEq,
            Token::NotEq => CmpOp::NotEq,
            Token::Lt    => CmpOp::Lt,
            Token::Gt    => CmpOp::Gt,
            Token::LtEq  => CmpOp::LtEq,
            Token::GtEq  => CmpOp::GtEq,
            tok => { eprintln!("Syntax error: expected comparison operator, got {:?}", tok); std::process::exit(1); }
        }
    }

    fn parse_condition(&mut self) -> Condition {
        self.expect(&Token::LParen);
        let left = self.parse_expr();
        let op = self.parse_cmp_op();
        let right = self.parse_expr();
        self.expect(&Token::RParen);
        Condition { left, op, right }
    }

    /// Parse a primary expression, then handle .field and (args) suffixes
    fn parse_expr(&mut self) -> Expr {
        let mut expr = match self.advance() {
            Token::StringLiteral(s) => Expr::StringLiteral(s),
            Token::NumberLiteral(n) => Expr::NumberLiteral(n),
            Token::Mina             => Expr::Identifier("mina".to_string()),
            Token::Identifier(name) => Expr::Identifier(name),
            tok => { eprintln!("Syntax error: expected expression, got {:?}", tok); std::process::exit(1); }
        };

        // handle chained .field and (args)
        loop {
            match self.peek() {
                Token::Dot => {
                    self.advance();
                    let field = self.expect_identifier();
                    expr = Expr::FieldAccess { object: Box::new(expr), field };
                }
                Token::LParen => {
                    let args = self.parse_args();
                    expr = Expr::Call { callee: Box::new(expr), args };
                }
                _ => break,
            }
        }
        expr
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        self.expect(&Token::LBrace);
        let mut stmts = Vec::new();
        while self.peek() != &Token::RBrace && self.peek() != &Token::EOF {
            stmts.push(self.parse_stmt());
        }
        self.expect(&Token::RBrace);
        stmts
    }

    fn parse_stmt(&mut self) -> Stmt {
        match self.peek().clone() {
            Token::Inombolo => {
                self.advance();
                let name = self.expect_identifier();
                self.expect(&Token::Equals);
                let value = match self.advance() {
                    Token::NumberLiteral(n) => n,
                    tok => { eprintln!("Type error: expected number, got {:?}", tok); std::process::exit(1); }
                };
                self.expect(&Token::Semicolon);
                Stmt::DeclareNumber { name, value }
            }
            Token::Ibala => {
                self.advance();
                let name = self.expect_identifier();
                self.expect(&Token::Equals);
                let value = match self.advance() {
                    Token::StringLiteral(s) => s,
                    tok => { eprintln!("Type error: expected string, got {:?}", tok); std::process::exit(1); }
                };
                self.expect(&Token::Semicolon);
                Stmt::DeclareString { name, value }
            }
            Token::Bhala => {
                self.advance();
                self.expect(&Token::LParen);
                let expr = self.parse_expr();
                self.expect(&Token::RParen);
                self.expect(&Token::Semicolon);
                Stmt::Print(expr)
            }
            Token::Uma => {
                self.advance();
                let condition = self.parse_condition();
                let body = self.parse_block();
                Stmt::If { condition, body }
            }
            Token::PhindaUma => {
                self.advance();
                let condition = self.parse_condition();
                let body = self.parse_block();
                Stmt::While { condition, body }
            }
            Token::Phinda => {
                self.advance();
                let body = self.parse_block();
                Stmt::Loop { body }
            }
            Token::Phuma => { self.advance(); self.expect(&Token::Semicolon); Stmt::Break }
            Token::Qhubeka => { self.advance(); self.expect(&Token::Semicolon); Stmt::Continue }
            Token::Phendusa => {
                self.advance();
                if self.peek() == &Token::Semicolon {
                    self.advance();
                    Stmt::Return(None)
                } else {
                    let expr = self.parse_expr();
                    self.expect(&Token::Semicolon);
                    Stmt::Return(Some(expr))
                }
            }
            // isigoqelo fnName(params) { body }
            Token::Isigoqelo => {
                self.advance();
                let name = self.expect_identifier();
                let params = self.parse_params();
                let body = self.parse_block();
                Stmt::FunctionDef { name, params, body }
            }
            // isakhi Name { fields and methods }
            Token::Isakhi => {
                self.advance();
                let name = self.expect_identifier();
                self.expect(&Token::LBrace);
                let mut fields = Vec::new();
                let mut methods = Vec::new();
                while self.peek() != &Token::RBrace && self.peek() != &Token::EOF {
                    match self.peek().clone() {
                        // field:  ibala IP  or  inombolo port
                        Token::Ibala | Token::Inombolo => {
                            let ty = self.parse_type();
                            let fname = self.expect_identifier();
                            fields.push(FieldDef { name: fname, ty });
                        }
                        // method: isigoqelo Start() { }
                        Token::Isigoqelo => {
                            self.advance();
                            let mname = self.expect_identifier();
                            let params = self.parse_params();
                            let body = self.parse_block();
                            methods.push(MethodDef { name: mname, params, body });
                        }
                        tok => {
                            eprintln!("Syntax error inside isakhi: unexpected {:?}", tok);
                            std::process::exit(1);
                        }
                    }
                }
                self.expect(&Token::RBrace);
                Stmt::StructDef { name, fields, methods }
            }
            Token::Umba => {
                self.advance();
                let struct_name = self.expect_identifier();
                let var_name = self.expect_identifier();
                self.expect(&Token::Semicolon);
                Stmt::Instantiate { struct_name, var_name }
            }
            // mina.field = expr;
            Token::Mina => {
                self.advance();
                self.expect(&Token::Dot);
                let field = self.expect_identifier();
                self.expect(&Token::Equals);
                let value = self.parse_expr();
                self.expect(&Token::Semicolon);
                Stmt::FieldAssign { field, value }
            }
            Token::EOF => {
                eprintln!("Syntax error: unexpected end of file");
                std::process::exit(1);
            }
            // expression statement or instance field assignment: var.field = val;
            _ => {
                let expr = self.parse_expr();
                // check for instance field assignment: var.field = expr;
                if self.peek() == &Token::Equals {
                    self.advance();
                    // expr must be a FieldAccess on a plain Identifier
                    match expr {
                        Expr::FieldAccess { object, field } => {
                            match *object {
                                Expr::Identifier(var_name) => {
                                    let value = self.parse_expr();
                                    self.expect(&Token::Semicolon);
                                    return Stmt::InstanceFieldAssign { var_name, field, value };
                                }
                                _ => {
                                    eprintln!("Syntax error: invalid assignment target");
                                    std::process::exit(1);
                                }
                            }
                        }
                        _ => {
                            eprintln!("Syntax error: invalid assignment target");
                            std::process::exit(1);
                        }
                    }
                }
                self.expect(&Token::Semicolon);
                Stmt::ExprStmt(expr)
            }
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.peek() != &Token::EOF {
            stmts.push(self.parse_stmt());
        }
        stmts
    }
}
