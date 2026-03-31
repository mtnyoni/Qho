#[cfg(test)]
use crate::lexer::{Lexer, Token};

fn lex(src: &str) -> Vec<Token> {
    let mut l = Lexer::new(src);
    l.tokenize()
}

#[test]
fn test_keywords() {
    let tokens = lex("bhala inombolo ibala uma phindaUma phinda phuma qhubeka isigoqelo isakhi umba bamba phendusa mina akulalutho");
    assert_eq!(tokens[0],  Token::Bhala);
    assert_eq!(tokens[1],  Token::Inombolo);
    assert_eq!(tokens[2],  Token::Ibala);
    assert_eq!(tokens[3],  Token::Uma);
    assert_eq!(tokens[4],  Token::PhindaUma);
    assert_eq!(tokens[5],  Token::Phinda);
    assert_eq!(tokens[6],  Token::Phuma);
    assert_eq!(tokens[7],  Token::Qhubeka);
    assert_eq!(tokens[8],  Token::Isigoqelo);
    assert_eq!(tokens[9],  Token::Isakhi);
    assert_eq!(tokens[10], Token::Umba);
    assert_eq!(tokens[11], Token::Bamba);
    assert_eq!(tokens[12], Token::Phendusa);
    assert_eq!(tokens[13], Token::Mina);
    assert_eq!(tokens[14], Token::Nil);
    assert_eq!(tokens[15], Token::EOF);
}

#[test]
fn test_string_literal() {
    let tokens = lex(r#""Sawubona""#);
    assert_eq!(tokens[0], Token::StringLiteral("Sawubona".to_string()));
    assert_eq!(tokens[1], Token::EOF);
}

#[test]
fn test_number_integer() {
    let tokens = lex("42");
    assert_eq!(tokens[0], Token::NumberLiteral(42.0));
}

#[test]
fn test_number_float() {
    let tokens = lex("3.14");
    assert_eq!(tokens[0], Token::NumberLiteral(3.14));
}

#[test]
fn test_identifier() {
    let tokens = lex("myVar");
    assert_eq!(tokens[0], Token::Identifier("myVar".to_string()));
}

#[test]
fn test_symbols() {
    let tokens = lex("= == != < > <= >= ( ) { } ; : , .");
    assert_eq!(tokens[0],  Token::Equals);
    assert_eq!(tokens[1],  Token::EqEq);
    assert_eq!(tokens[2],  Token::NotEq);
    assert_eq!(tokens[3],  Token::Lt);
    assert_eq!(tokens[4],  Token::Gt);
    assert_eq!(tokens[5],  Token::LtEq);
    assert_eq!(tokens[6],  Token::GtEq);
    assert_eq!(tokens[7],  Token::LParen);
    assert_eq!(tokens[8],  Token::RParen);
    assert_eq!(tokens[9],  Token::LBrace);
    assert_eq!(tokens[10], Token::RBrace);
    assert_eq!(tokens[11], Token::Semicolon);
    assert_eq!(tokens[12], Token::Colon);
    assert_eq!(tokens[13], Token::Comma);
    assert_eq!(tokens[14], Token::Dot);
    assert_eq!(tokens[15], Token::EOF);
}

#[test]
fn test_skips_line_comments() {
    let tokens = lex("// this is a comment\nbhala");
    assert_eq!(tokens[0], Token::Bhala);
    assert_eq!(tokens[1], Token::EOF);
}

#[test]
fn test_skips_whitespace() {
    let tokens = lex("   \t\n  bhala");
    assert_eq!(tokens[0], Token::Bhala);
}

#[test]
fn test_full_declaration() {
    let tokens = lex(r#"inombolo x = 5;"#);
    assert_eq!(tokens[0], Token::Inombolo);
    assert_eq!(tokens[1], Token::Identifier("x".to_string()));
    assert_eq!(tokens[2], Token::Equals);
    assert_eq!(tokens[3], Token::NumberLiteral(5.0));
    assert_eq!(tokens[4], Token::Semicolon);
}

#[test]
fn test_string_declaration() {
    let tokens = lex(r#"ibala name = "Tawanda";"#);
    assert_eq!(tokens[0], Token::Ibala);
    assert_eq!(tokens[1], Token::Identifier("name".to_string()));
    assert_eq!(tokens[2], Token::Equals);
    assert_eq!(tokens[3], Token::StringLiteral("Tawanda".to_string()));
    assert_eq!(tokens[4], Token::Semicolon);
}

#[test]
fn test_method_call_tokens() {
    let tokens = lex("s.Start();");
    assert_eq!(tokens[0], Token::Identifier("s".to_string()));
    assert_eq!(tokens[1], Token::Dot);
    assert_eq!(tokens[2], Token::Identifier("Start".to_string()));
    assert_eq!(tokens[3], Token::LParen);
    assert_eq!(tokens[4], Token::RParen);
    assert_eq!(tokens[5], Token::Semicolon);
}
