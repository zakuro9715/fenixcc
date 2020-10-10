use crate::token::{Pos, Symbol, Token, TokenKind};
use std::char;
use std::iter::Iterator;
use std::vec::Vec;

pub struct Lexer {
    pos: Pos,
    source: Vec<char>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    Message(Pos, String),
    EOF,
}

type Result<T> = std::result::Result<T, Error>;

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Self {
            pos: Pos::head(),
            source: source.chars().collect(),
        }
    }

    fn eof(&self) -> bool {
        self.pos.offset >= self.source.len()
    }

    fn peek_char(&mut self) -> Result<&char> {
        self.peek_char_i(0)
    }

    fn peek_char_i(&mut self, i: usize) -> Result<&char> {
        self.source.get(self.pos.offset + i).ok_or(Error::EOF)
    }

    fn skip_whitespaces(&mut self) -> Result<()> {
        while !self.eof() && self.peek_char()?.is_whitespace() {
            if let Err(err) = self.consume() {
                return Err(err)
            }
        }
        Ok(())
    }

    fn read_integer(&mut self) -> Result<Token> {
        let start_pos = self.pos;
        while !self.eof() && self.peek_char()?.is_ascii_digit() {
            self.consume()?
        }
        let text: String = self.source[start_pos.offset..self.pos.offset]
            .iter()
            .collect();
        let int = text.parse::<i64>().unwrap();
        Ok(Token::new(TokenKind::Int(int), start_pos))
    }

    fn new_line(&mut self) -> Result<()> {
        let is_cr = *self.peek_char()? == '\r';
        self.pos.offset += 1;
        if is_cr && *self.peek_char()? == '\n' {
            self.pos.offset += 1;
        }
        self.pos.line += 1;
        self.pos.col = 1;
        Ok(())
    }

    fn consume(&mut self) -> Result<()> {
        if let '\n' | '\r' = self.peek_char()? {
            self.new_line()
        } else {
            self.pos.offset += 1;
            self.pos.col += 1;
            Ok(())
        }
    }

    pub fn next(&mut self) -> Result<Token> {
        self.skip_whitespaces()?;
        if self.eof() {
            return Err(Error::EOF);
        }

        let pos = self.pos;
        match self.peek_char()? {
            '+' => {
                let _ = self.consume();
                Ok(Token::new(TokenKind::Symbol(Symbol::Plus), pos))
            }
            '-' => {
                let _ = self.consume();
                Ok(Token::new(TokenKind::Symbol(Symbol::Minus), pos))
            }
            c if c.is_ascii_digit() => {
                self.read_integer()
            }
            c => Err(Error::Message(pos, format!("Invald char {}", c))),
        }
    }
}

#[test]
fn test_lexer() {
    let mut lexer = Lexer::new("1 + 2 -  3\n0".to_string());
    assert_eq!(
        lexer.next(),
        Ok(Token::new(
            TokenKind::Int(1),
            Pos::new(0, 1, 1)
        ))
    );
    assert_eq!(
        lexer.next(),
        Ok(Token::new(
            TokenKind::Symbol(Symbol::Plus),
            Pos::new(2, 1, 3)
        ))
    );
    assert_eq!(
        lexer.next(),
        Ok(Token::new(
            TokenKind::Int(2),
            Pos::new(4, 1, 5)
        ))
    );
    assert_eq!(
        lexer.next(),
        Ok(Token::new(
            TokenKind::Symbol(Symbol::Minus),
            Pos::new(6, 1, 7)
        ))
    );
    assert_eq!(
        lexer.next(),
        Ok(Token::new(
            TokenKind::Int(3),
            Pos::new(9, 1, 10)
        ))
    );
    assert_eq!(
        lexer.next(),
        Ok(Token::new(
            TokenKind::Int(0),
            Pos::new(11, 2, 1)
        ))
    );
    assert_eq!(
        lexer.next(),
        Err(Error::EOF),
    );
}
