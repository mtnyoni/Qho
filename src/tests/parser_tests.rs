#[cfg(test)]
use crate::lexer::Lexer;
use crate::parser::{Parser, Stmt, Expr, CmpOp};

fn parse(src: &str) -> Vec<Stmt> {
    let mut l = Lexer::new(src);
    let tokens = l.tokenize();
    let mut p = Parser::new(tokens);
    p.parse()
}

#[test]
fn test_parse_number_declaration() {
    let stmts = parse("inombolo x = 10;");
    assert!(matches!(&stmts[0], Stmt::DeclareNumber { name, value } if name == "x" && *value == 10.0));
}

#[test]
fn test_parse_string_declaration() {
    let stmts = parse(r#"ibala name = "Tawanda";"#);
    assert!(matches!(&stmts[0], Stmt::DeclareString { name, value } if name == "name" && value == "Tawanda"));
}

#[test]
fn test_parse_bamba_declaration() {
    let stmts = parse(r#"yenza x = "hello";"#);
    assert!(matches!(&stmts[0], Stmt::DeclareVal { name, .. } if name == "x"));
}

#[test]
fn test_parse_print_string() {
    let stmts = parse(r#"bhala("hello");"#);
    assert!(matches!(&stmts[0], Stmt::Print(Expr::StringLiteral(s)) if s == "hello"));
}

#[test]
fn test_parse_print_identifier() {
    let stmts = parse("bhala(x);");
    assert!(matches!(&stmts[0], Stmt::Print(Expr::Identifier(n)) if n == "x"));
}

#[test]
fn test_parse_if() {
    let stmts = parse("uma (x > 5) { bhala(x); }");
    match &stmts[0] {
        Stmt::If { condition, body } => {
            assert!(matches!(condition.op, CmpOp::Gt));
            assert_eq!(body.len(), 1);
        }
        _ => panic!("expected If"),
    }
}

#[test]
fn test_parse_while() {
    let stmts = parse("phindaUma (x < 10) { bhala(x); }");
    assert!(matches!(&stmts[0], Stmt::While { .. }));
}

#[test]
fn test_parse_loop_with_break() {
    let stmts = parse("phinda { phuma; }");
    match &stmts[0] {
        Stmt::Loop { body } => {
            assert!(matches!(body[0], Stmt::Break));
        }
        _ => panic!("expected Loop"),
    }
}

#[test]
fn test_parse_continue() {
    let stmts = parse("phinda { qhubeka; }");
    match &stmts[0] {
        Stmt::Loop { body } => {
            assert!(matches!(body[0], Stmt::Continue));
        }
        _ => panic!("expected Loop with Continue"),
    }
}

#[test]
fn test_parse_function_def() {
    let stmts = parse("isigoqelo greet(name: ibala) { bhala(name); }");
    match &stmts[0] {
        Stmt::FunctionDef { name, params, body } => {
            assert_eq!(name, "greet");
            assert_eq!(params.len(), 1);
            assert_eq!(params[0].name, "name");
            assert_eq!(body.len(), 1);
        }
        _ => panic!("expected FunctionDef"),
    }
}

#[test]
fn test_parse_function_call() {
    let stmts = parse(r#"greet("Tawanda");"#);
    assert!(matches!(&stmts[0], Stmt::ExprStmt(Expr::Call { .. })));
}

#[test]
fn test_parse_return_value() {
    let stmts = parse("isigoqelo add(x: inombolo) { phendukisa x; }");
    match &stmts[0] {
        Stmt::FunctionDef { body, .. } => {
            assert!(matches!(&body[0], Stmt::Return(Some(_))));
        }
        _ => panic!("expected FunctionDef"),
    }
}

#[test]
fn test_parse_return_empty() {
    let stmts = parse("isigoqelo noop() { phendukisa; }");
    match &stmts[0] {
        Stmt::FunctionDef { body, .. } => {
            assert!(matches!(&body[0], Stmt::Return(None)));
        }
        _ => panic!("expected FunctionDef"),
    }
}

#[test]
fn test_parse_struct_def() {
    let stmts = parse("isakhi Server { ibala IP inombolo port }");
    match &stmts[0] {
        Stmt::StructDef { name, fields, methods } => {
            assert_eq!(name, "Server");
            assert_eq!(fields.len(), 2);
            assert_eq!(fields[0].name, "IP");
            assert_eq!(fields[1].name, "port");
            assert_eq!(methods.len(), 0);
        }
        _ => panic!("expected StructDef"),
    }
}

#[test]
fn test_parse_struct_with_method() {
    let stmts = parse("isakhi Server { ibala IP isigoqelo Start() { bhala(mina.IP); } }");
    match &stmts[0] {
        Stmt::StructDef { methods, .. } => {
            assert_eq!(methods.len(), 1);
            assert_eq!(methods[0].name, "Start");
        }
        _ => panic!("expected StructDef"),
    }
}

#[test]
fn test_parse_instantiate() {
    let stmts = parse("bumba Server s;");
    assert!(matches!(&stmts[0], Stmt::Instantiate { struct_name, var_name }
        if struct_name == "Server" && var_name == "s"));
}

#[test]
fn test_parse_instance_field_assign() {
    let stmts = parse(r#"s.IP = "127.0.0.1";"#);
    assert!(matches!(&stmts[0], Stmt::InstanceFieldAssign { var_name, field, .. }
        if var_name == "s" && field == "IP"));
}

#[test]
fn test_parse_mina_field_assign() {
    let stmts = parse("isakhi S { ibala x isigoqelo Set() { mina.x = \"hi\"; } }");
    match &stmts[0] {
        Stmt::StructDef { methods, .. } => {
            assert!(matches!(&methods[0].body[0], Stmt::FieldAssign { field, .. } if field == "x"));
        }
        _ => panic!("expected StructDef"),
    }
}

#[test]
fn test_parse_method_call() {
    let stmts = parse("s.Start();");
    assert!(matches!(&stmts[0], Stmt::ExprStmt(Expr::Call { .. })));
}

#[test]
fn test_parse_nil_in_condition() {
    let stmts = parse("uma (x == akulalutho) { bhala(x); }");
    assert!(matches!(&stmts[0], Stmt::If { .. }));
}

#[test]
fn test_parse_comparison_operators() {
    let ops = vec![
        ("uma (x == 1) {}", CmpOp::EqEq),
        ("uma (x != 1) {}", CmpOp::NotEq),
        ("uma (x < 1) {}",  CmpOp::Lt),
        ("uma (x > 1) {}",  CmpOp::Gt),
        ("uma (x <= 1) {}", CmpOp::LtEq),
        ("uma (x >= 1) {}", CmpOp::GtEq),
    ];
    for (src, expected_op) in ops {
        let stmts = parse(src);
        match &stmts[0] {
            Stmt::If { condition, .. } => {
                assert!(matches!(&condition.op, op if std::mem::discriminant(op) == std::mem::discriminant(&expected_op)));
            }
            _ => panic!("expected If for: {}", src),
        }
    }
}
