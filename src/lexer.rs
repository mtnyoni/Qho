/// Tokens for the Ndebele language
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Bhala,       // print
    Inombolo,    // float64 type
    Ibala,       // string type
    Uma,         // if
    PhindaUma,   // while
    Phinda,      // loop (infinite)
    Phuma,       // break
    Qhubeka,     // continue
    Isigoqelo,   // function
    Isakhi,      // struct
    Umba,        // instantiate a struct
    Bamba,       // generic value binding (val)
    Phendusa,    // return
    Mina,        // self (mina inside methods)
    Nil,         // nil literal

    // Literals
    StringLiteral(String),
    NumberLiteral(f64),

    // Identifier (variable name / struct name / function name)
    Identifier(String),

    // Symbols
    Equals,    // =
    EqEq,      // ==
    NotEq,     // !=
    Lt,        // <
    Gt,        // >
    LtEq,      // <=
    GtEq,      // >=
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }
    Semicolon, // ;
    Colon,     // :
    Comma,     // ,
    Dot,       // .

    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer { input: source.chars().collect(), pos: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.input.get(self.pos).copied();
        self.pos += 1;
        ch
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            // skip whitespace
            while matches!(self.peek(), Some(c) if c.is_whitespace()) {
                self.advance();
            }
            // skip // line comments
            if self.peek() == Some('/') && self.input.get(self.pos + 1).copied() == Some('/') {
                while !matches!(self.peek(), Some('\n') | None) {
                    self.advance();
                }
            } else {
                break;
            }
        }
    }

    fn read_string(&mut self) -> String {
        let mut s = String::new();
        loop {
            match self.advance() {
                Some('"') | None => break,
                Some(c) => s.push(c),
            }
        }
        s
    }

    fn read_number(&mut self, first: char) -> f64 {
        let mut num = String::from(first);
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '.' { num.push(c); self.advance(); }
            else { break; }
        }
        num.parse().unwrap_or_else(|_| {
            eprintln!("Lexer error: invalid number '{}'", num);
            std::process::exit(1);
        })
    }

    fn read_identifier(&mut self, first: char) -> String {
        let mut ident = String::from(first);
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' { ident.push(c); self.advance(); }
            else { break; }
        }
        ident
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            self.skip_whitespace_and_comments();
            match self.peek() {
                None => { tokens.push(Token::EOF); break; }
                Some('"') => { self.advance(); tokens.push(Token::StringLiteral(self.read_string())); }
                Some('(') => { self.advance(); tokens.push(Token::LParen); }
                Some(')') => { self.advance(); tokens.push(Token::RParen); }
                Some('{') => { self.advance(); tokens.push(Token::LBrace); }
                Some('}') => { self.advance(); tokens.push(Token::RBrace); }
                Some(';') => { self.advance(); tokens.push(Token::Semicolon); }
                Some(':') => { self.advance(); tokens.push(Token::Colon); }
                Some(',') => { self.advance(); tokens.push(Token::Comma); }
                Some('.') => { self.advance(); tokens.push(Token::Dot); }
                Some('=') => {
                    self.advance();
                    if self.peek() == Some('=') { self.advance(); tokens.push(Token::EqEq); }
                    else { tokens.push(Token::Equals); }
                }
                Some('!') => {
                    self.advance();
                    if self.peek() == Some('=') { self.advance(); tokens.push(Token::NotEq); }
                    else { eprintln!("Lexer error: unexpected '!'"); std::process::exit(1); }
                }
                Some('<') => {
                    self.advance();
                    if self.peek() == Some('=') { self.advance(); tokens.push(Token::LtEq); }
                    else { tokens.push(Token::Lt); }
                }
                Some('>') => {
                    self.advance();
                    if self.peek() == Some('=') { self.advance(); tokens.push(Token::GtEq); }
                    else { tokens.push(Token::Gt); }
                }
                Some(c) if c.is_ascii_digit() => {
                    self.advance();
                    tokens.push(Token::NumberLiteral(self.read_number(c)));
                }
                Some(c) if c.is_alphabetic() || c == '_' => {
                    self.advance();
                    let ident = self.read_identifier(c);
                    let tok = match ident.as_str() {
                        "bhala"      => Token::Bhala,
                        "inombolo"   => Token::Inombolo,
                        "ibala"      => Token::Ibala,
                        "uma"        => Token::Uma,
                        "phindaUma"  => Token::PhindaUma,
                        "phinda"     => Token::Phinda,
                        "phuma"      => Token::Phuma,
                        "qhubeka"    => Token::Qhubeka,
                        "isigoqelo"  => Token::Isigoqelo,
                        "isakhi"     => Token::Isakhi,
                        "umba"       => Token::Umba,
                        "bamba"      => Token::Bamba,
                        "phendusa"   => Token::Phendusa,
                        "mina"       => Token::Mina,
                        "akulalutho" => Token::Nil,
                        _            => Token::Identifier(ident),
                    };
                    tokens.push(tok);
                }
                Some(c) => {
                    eprintln!("Lexer error: unexpected character '{}'", c);
                    std::process::exit(1);
                }
            }
        }
        tokens
    }
}
