use crate::token::{Pos, Symbol, Token, TokenKind};
use std::char;
use std::iter::Iterator;
use std::vec::Vec;

pub struct Lexer {
    pos: Pos,
    source: Vec<char>,
    terminated: bool,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            pos: Pos::head(),
            source: source.chars().collect(),
            terminated: false,
        }
    }

    fn eof(&self) -> bool {
        self.pos.offset >= self.source.len()
    }

    fn head(&mut self) -> char {
        self.source[self.pos.offset]
    }

    fn skip_whitespace(&mut self) {
        while !self.eof() && self.head().is_whitespace() {
            if self.head() == '\n' {
                println!("EOF");
                self.pos.offset += 1;
                self.pos.line += 1;
                self.pos.col = 1;
            } else {
                self.consume();
            }
        }
    }

    fn read_integer(&mut self) -> Token {
        let start_pos = self.pos;
        while !self.eof() && self.head().is_ascii_digit() {
            self.consume()
        }
        let text: String = self.source[start_pos.offset..self.pos.offset]
            .into_iter()
            .collect();
        let int = text.parse::<i64>().unwrap();
        Token::new(format!("{}", text), TokenKind::Int(int), start_pos)
    }

    fn consume(&mut self) {
        self.pos.offset += 1;
        self.pos.col += 1;
    }

    fn tokenize(&mut self) -> Token {
        let head = self.head();
        let mut skip_consume = false;

        let token = match head {
            '+' => Token::new("+".to_string(), TokenKind::Symbol(Symbol::Plus), self.pos),
            '-' => Token::new("-".to_string(), TokenKind::Symbol(Symbol::Minus), self.pos),
            '\n' => Token::new("\n".to_string(), TokenKind::EOF, self.pos),
            _ if head.is_ascii_digit() => {
                skip_consume = true;
                self.read_integer()
            }
            _ => Token::new("ERROR".to_string(), TokenKind::Error, self.pos),
        };

        if !skip_consume {
            self.consume();
        }

        token
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.skip_whitespace();
        return if self.eof() {
            if self.terminated {
                None
            } else {
                Some(Token::new("EOF".to_string(), TokenKind::EOF, self.pos))
            }
        } else {
            Some(self.tokenize())
        };
    }
}

#[test]
fn test_lexer() {
    let mut lexer = Lexer::new("1 + 2 -  3\n0".to_string());
    assert_eq!(
        lexer.next(),
        Some(Token::new(
            "1".to_string(),
            TokenKind::Int(1),
            Pos::new(0, 1, 1)
        ))
    );
    assert_eq!(
        lexer.next(),
        Some(Token::new(
            "+".to_string(),
            TokenKind::Symbol(Symbol::Plus),
            Pos::new(2, 1, 3)
        ))
    );
    assert_eq!(
        lexer.next(),
        Some(Token::new(
            "2".to_string(),
            TokenKind::Int(2),
            Pos::new(4, 1, 5)
        ))
    );
    assert_eq!(
        lexer.next(),
        Some(Token::new(
            "-".to_string(),
            TokenKind::Symbol(Symbol::Minus),
            Pos::new(6, 1, 7)
        ))
    );
    assert_eq!(
        lexer.next(),
        Some(Token::new(
            "3".to_string(),
            TokenKind::Int(3),
            Pos::new(9, 1, 10)
        ))
    );
    assert_eq!(
        lexer.next(),
        Some(Token::new(
            "0".to_string(),
            TokenKind::Int(0),
            Pos::new(11, 2, 1)
        ))
    );
    assert_eq!(
        lexer.next(),
        Some(Token::new(
            "EOF".to_string(),
            TokenKind::EOF,
            Pos::new(12, 2, 2)
        ))
    );
}
