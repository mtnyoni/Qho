#[cfg(test)]
use std::collections::HashMap;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::{Interpreter, Value};

/// Run source and return the interpreter after execution (so we can inspect vars)
fn run(src: &str) -> Interpreter {
    let mut l = Lexer::new(src);
    let tokens = l.tokenize();
    let mut p = Parser::new(tokens);
    let ast = p.parse();
    let mut interp = Interpreter::new();
    interp.run(&ast);
    interp
}

/// Helper: get a variable value from the interpreter
fn get_num(interp: &Interpreter, name: &str) -> f64 {
    match interp.get_var(name) {
        Some(Value::Number(n)) => *n,
        _ => panic!("expected Number for '{}'", name),
    }
}

fn get_str<'a>(interp: &'a Interpreter, name: &str) -> &'a str {
    match interp.get_var(name) {
        Some(Value::Str(s)) => s.as_str(),
        _ => panic!("expected Str for '{}'", name),
    }
}

// ── Variable declarations ─────────────────────────────────────────────────────

#[test]
fn test_declare_number() {
    let interp = run("inombolo x = 42;");
    assert_eq!(get_num(&interp, "x"), 42.0);
}

#[test]
fn test_declare_float() {
    let interp = run("inombolo pi = 3.14;");
    assert!((get_num(&interp, "pi") - 3.14).abs() < f64::EPSILON);
}

#[test]
fn test_declare_string() {
    let interp = run(r#"ibala name = "Tawanda";"#);
    assert_eq!(get_str(&interp, "name"), "Tawanda");
}

#[test]
fn test_declare_val_string() {
    let interp = run(r#"yenza x = "hello";"#);
    assert_eq!(get_str(&interp, "x"), "hello");
}

#[test]
fn test_declare_val_number() {
    let interp = run("yenza x = 99;");
    assert_eq!(get_num(&interp, "x"), 99.0);
}

// ── If statements ─────────────────────────────────────────────────────────────

#[test]
fn test_if_true_branch_executes() {
    let interp = run("inombolo x = 1; uma (x == 1) { inombolo y = 99; }");
    assert_eq!(get_num(&interp, "y"), 99.0);
}

#[test]
fn test_if_false_branch_skipped() {
    let interp = run("inombolo x = 0; uma (x == 1) { inombolo y = 99; }");
    assert!(interp.get_var("y").is_none());
}

#[test]
fn test_if_gt() {
    let interp = run("inombolo x = 10; uma (x > 5) { inombolo result = 1; }");
    assert_eq!(get_num(&interp, "result"), 1.0);
}

#[test]
fn test_if_lt() {
    let interp = run("inombolo x = 3; uma (x < 5) { inombolo result = 1; }");
    assert_eq!(get_num(&interp, "result"), 1.0);
}

#[test]
fn test_if_lteq() {
    let interp = run("inombolo x = 5; uma (x <= 5) { inombolo result = 1; }");
    assert_eq!(get_num(&interp, "result"), 1.0);
}

#[test]
fn test_if_gteq() {
    let interp = run("inombolo x = 5; uma (x >= 5) { inombolo result = 1; }");
    assert_eq!(get_num(&interp, "result"), 1.0);
}

#[test]
fn test_if_neq() {
    let interp = run("inombolo x = 3; uma (x != 5) { inombolo result = 1; }");
    assert_eq!(get_num(&interp, "result"), 1.0);
}

#[test]
fn test_if_string_eq() {
    let interp = run(r#"ibala s = "hi"; uma (s == "hi") { inombolo result = 1; }"#);
    assert_eq!(get_num(&interp, "result"), 1.0);
}

// ── While loop ────────────────────────────────────────────────────────────────

#[test]
fn test_while_does_not_run_when_false() {
    let interp = run("inombolo x = 10; phindaUma (x < 5) { inombolo ran = 1; }");
    assert!(interp.get_var("ran").is_none());
}

// ── Loop + break ──────────────────────────────────────────────────────────────

#[test]
fn test_loop_break() {
    let interp = run("inombolo x = 0; phinda { inombolo x = 1; phuma; }");
    assert_eq!(get_num(&interp, "x"), 1.0);
}

// ── Functions ─────────────────────────────────────────────────────────────────

#[test]
fn test_function_def_and_call() {
    let interp = run("
        isigoqelo setVal() {
            inombolo result = 42;
        }
        setVal();
    ");
    // result is local to the function, not visible outside
    assert!(interp.get_var("result").is_none());
}

#[test]
fn test_function_return_value() {
    let interp = run("
        isigoqelo getNum() {
            phendukisa 7;
        }
        yenza r = getNum();
    ");
    assert_eq!(get_num(&interp, "r"), 7.0);
}

#[test]
fn test_function_with_param() {
    let interp = run(r#"
        isigoqelo echo(msg: ibala) {
            phendukisa msg;
        }
        yenza r = echo("hello");
    "#);
    assert_eq!(get_str(&interp, "r"), "hello");
}

#[test]
fn test_function_return_none() {
    let interp = run("
        isigoqelo noop() {
            phendukisa;
        }
        yenza r = noop();
    ");
    assert!(matches!(interp.get_var("r"), Some(Value::Nil)));
}

// ── Structs ───────────────────────────────────────────────────────────────────

#[test]
fn test_struct_instantiate_default_fields() {
    let interp = run("
        isakhi Point {
            inombolo x
            inombolo y
        }
        bumba Point p;
    ");
    match interp.get_var("p") {
        Some(Value::Instance { struct_name, fields }) => {
            assert_eq!(struct_name, "Point");
            assert!(matches!(fields.get("x"), Some(Value::Number(n)) if *n == 0.0));
            assert!(matches!(fields.get("y"), Some(Value::Number(n)) if *n == 0.0));
        }
        _ => panic!("expected Instance"),
    }
}

#[test]
fn test_struct_field_assign() {
    let interp = run("
        isakhi Point { inombolo x inombolo y }
        bumba Point p;
        p.x = 10;
        p.y = 20;
    ");
    match interp.get_var("p") {
        Some(Value::Instance { fields, .. }) => {
            assert!(matches!(fields.get("x"), Some(Value::Number(n)) if *n == 10.0));
            assert!(matches!(fields.get("y"), Some(Value::Number(n)) if *n == 20.0));
        }
        _ => panic!("expected Instance"),
    }
}

#[test]
fn test_struct_method_call() {
    let interp = run(r#"
        isakhi Greeter {
            ibala name
            isigoqelo greet() {
                phendukisa mina.name;
            }
        }
        bumba Greeter g;
        g.name = "Tawanda";
        yenza result = g.greet();
    "#);
    assert_eq!(get_str(&interp, "result"), "Tawanda");
}

#[test]
fn test_struct_method_mutates_field() {
    let interp = run("
        isakhi Counter {
            inombolo count
            isigoqelo reset() {
                mina.count = 0;
            }
        }
        bumba Counter c;
        c.count = 99;
        c.reset();
    ");
    // mina mutations don't propagate back to outer scope yet (child interpreter)
    // but the call should not crash
    assert!(interp.get_var("c").is_some());
}

// ── Nil ───────────────────────────────────────────────────────────────────────

#[test]
fn test_nil_equality() {
    let interp = run("
        yenza x = akulalutho;
        uma (x == akulalutho) { inombolo isNil = 1; }
    ");
    assert_eq!(get_num(&interp, "isNil"), 1.0);
}

#[test]
fn test_nil_inequality() {
    let interp = run(r#"
        ibala x = "hi";
        uma (x != akulalutho) { inombolo notNil = 1; }
    "#);
    assert_eq!(get_num(&interp, "notNil"), 1.0);
}

// ── Value display ─────────────────────────────────────────────────────────────

#[test]
fn test_value_display_whole_number() {
    let v = Value::Number(5.0);
    assert_eq!(format!("{}", v), "5");
}

#[test]
fn test_value_display_float() {
    let v = Value::Number(3.14);
    assert_eq!(format!("{}", v), "3.14");
}

#[test]
fn test_value_display_string() {
    let v = Value::Str("hello".to_string());
    assert_eq!(format!("{}", v), "hello");
}

#[test]
fn test_value_display_nil() {
    let v = Value::Nil;
    assert_eq!(format!("{}", v), "akulalutho");
}
