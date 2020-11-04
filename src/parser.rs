use crate::ast::AST;
#[cfg(test)]
use crate::Symbol;
use crate::{ast, sym, Token, TokenKind};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Error {
    Message(Token, String),
    EOF,
}

pub type Result<T> = std::result::Result<T, Error>;

use std::iter::{Iterator, Peekable};
pub struct Parser<Tokens: Iterator<Item = Token>> {
    pub tokens: Peekable<Tokens>,
}

impl<Tokens: Iterator<Item = Token>> Parser<Tokens> {
    pub fn new(tokens: Tokens) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<AST> {
        self.parse_expr()
    }

    fn peek_token(&mut self) -> &Token {
        self.tokens.peek().unwrap()
    }

    fn next_token(&mut self) -> Token {
        self.tokens.next().unwrap()
    }

    fn parse_expr(&mut self) -> Result<AST> {
        let mut ast = self.parse_value()?;
        while matches!(self.peek_token().kind, sym!(Plus) | sym!(Minus)) {
            let op = self.read_symbol()?;
            let rhs = self.parse_value()?;
            ast = ast!(new_binary_expr, ast, op, rhs);
        }
        Ok(ast)
    }

    fn read_token_with_match<F: Fn(&Token) -> bool>(&mut self, matches: F) -> Result<Token> {
        let tok = self.next_token();
        match tok.kind {
            _ if matches(&tok) => Ok(tok),
            _ => Err(Error::Message(tok, "Unexpected Token".to_string())),
        }
    }

    fn read_symbol(&mut self) -> Result<Token> {
        self.read_token_with_match(|t| matches!(t.kind, TokenKind::Symbol(_)))
    }

    fn parse_value(&mut self) -> Result<AST> {
        self.parse_int()
    }

    fn parse_int(&mut self) -> Result<AST> {
        let tok = self.read_token_with_match(|t| matches!(t.kind, TokenKind::Int(_)))?;
        Ok(ast!(new_literal, tok))
    }
}

#[cfg(test)]
use crate::tok;
#[cfg(test)]
use crate::token::Loc;

#[test]
fn test_expr() {
    let tokens = vec![
        tok!(new_int, 0, Loc::new(0, 1, 1)),
        tok!(new_symbol, Symbol::Plus, Loc::new(1, 1, 2)),
        tok!(new_int, 0, Loc::new(2, 1, 3)),
        tok!(new_symbol, Symbol::Minus, Loc::new(3, 1, 4)),
        tok!(new_int, 0, Loc::new(4, 1, 5)),
        tok!(new_eof, Loc::new(4, 1, 6)),
    ];
    let v = Parser::new(tokens.clone().into_iter()).parse().unwrap();
    assert_eq!(
        format!("{:?}", v),
        format!(
            "{:?}",
            ast!(
                new_binary_expr,
                ast!(
                    new_binary_expr,
                    ast!(new_literal, tokens[0].clone()),
                    tokens[1].clone(),
                    ast!(new_literal, tokens[2].clone()),
                ),
                tokens[3].clone(),
                ast!(new_literal, tokens[4].clone()),
            )
        ),
    );
}
