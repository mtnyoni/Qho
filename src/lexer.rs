/// Tokens for the Ndebele language
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keyword: print to console
    FakaAmabalaKuScreen,
    // A string literal e.g. "hello"
    StringLiteral(String),
    // ( ) 
    LParen,
    RParen,
    // End of statement
    Semicolon,
    // End of file
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            input: source.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.input.get(self.pos).copied();
        self.pos += 1;
        ch
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_whitespace()) {
            self.advance();
        }
    }

    fn read_string(&mut self) -> String {
        // opening quote already consumed
        let mut s = String::new();
        loop {
            match self.advance() {
                Some('"') | None => break,
                Some(c) => s.push(c),
            }
        }
        s
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }
        ident
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            self.skip_whitespace();
            match self.peek() {
                None => {
                    tokens.push(Token::EOF);
                    break;
                }
                Some('"') => {
                    self.advance(); // consume opening quote
                    let s = self.read_string();
                    tokens.push(Token::StringLiteral(s));
                }
                Some('(') => { self.advance(); tokens.push(Token::LParen); }
                Some(')') => { self.advance(); tokens.push(Token::RParen); }
                Some(';') => { self.advance(); tokens.push(Token::Semicolon); }
                Some(c) if c.is_alphabetic() => {
                    let ident = self.read_identifier();
                    let tok = match ident.as_str() {
                        "fakaAmabalaKuScreen" => Token::FakaAmabalaKuScreen,
                        _ => panic!("Unknown identifier: {}", ident),
                    };
                    tokens.push(tok);
                }
                Some(c) => panic!("Unexpected character: {:?}", c),
            }
        }
        tokens
    }
}
