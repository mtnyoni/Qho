use crate::lexer::Token;

/// AST nodes
#[derive(Debug)]
pub enum Stmt {
    /// fakaAmabalaKuScreen("hello");
    Print(Expr),
}

#[derive(Debug)]
pub enum Expr {
    StringLiteral(String),
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> &Token {
        let tok = &self.tokens[self.pos];
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: &Token) {
        let tok = self.advance().clone();
        if &tok != expected {
            panic!("Expected {:?}, got {:?}", expected, tok);
        }
    }

    fn parse_expr(&mut self) -> Expr {
        match self.advance().clone() {
            Token::StringLiteral(s) => Expr::StringLiteral(s),
            tok => panic!("Expected expression, got {:?}", tok),
        }
    }

    fn parse_stmt(&mut self) -> Stmt {
        match self.advance().clone() {
            Token::FakaAmabalaKuScreen => {
                self.expect(&Token::LParen);
                let expr = self.parse_expr();
                self.expect(&Token::RParen);
                self.expect(&Token::Semicolon);
                Stmt::Print(expr)
            }
            tok => panic!("Unexpected token: {:?}", tok),
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
