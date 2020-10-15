use crate::tok;
use crate::token::{Pos, Symbol, Token, TokenKind};
use std::char;
use std::iter::Iterator;
use std::vec::Vec;

pub struct Lexer {
    pos: Pos,
    source: Vec<char>,
}

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

    fn peek_char(&mut self) -> char {
        if let Some(c) = self.source.get(self.pos.offset) {
            *c
        } else {
            char::REPLACEMENT_CHARACTER
        }
    }

    fn skip_whitespaces(&mut self) {
        let _ = self.read_while(|c| c.is_whitespace());
    }

    fn read_while<F: Fn(char) -> bool>(&mut self, f: F) -> String {
        let start_pos = self.pos;
        while !self.eof() && f(self.peek_char()) {
            self.consume()
        }
        self.source[start_pos.offset..self.pos.offset]
            .iter()
            .collect()
    }

    fn read_integer(&mut self) -> Token {
        let pos = self.pos;
        let int = self
            .read_while(|b| b.is_ascii_digit())
            .parse::<i64>()
            .unwrap();
        Token::new(TokenKind::Int(int), pos)
    }

    fn consume(&mut self) {
        if self.eof() {
            return;
        }
        match self.peek_char() {
            '\n' => {
                self.pos.offset += 1;
                self.pos.line += 1;
                self.pos.col = 1;
            }
            '\r' => {
                self.pos.offset += 1;
                self.pos.line += 1;
                self.pos.col = 1;
                // CRLF
                if self.peek_char() == '\n' {
                    self.pos.offset += 1
                }
            }
            _ => {
                self.pos.offset += 1;
                self.pos.col += 1;
            }
        };
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.skip_whitespaces();

        let pos = self.pos;

        if self.eof() {
            return Some(tok!(new_eof, pos));
        }

        Some(match self.peek_char() {
            '+' => {
                let _ = self.consume();
                tok!(new_symbol, Symbol::Plus, pos)
            }
            '-' => {
                let _ = self.consume();
                tok!(new_symbol, Symbol::Minus, pos)
            }
            c if c.is_ascii_digit() => self.read_integer(),
            c => {
                let _ = self.consume();
                tok!(new_invalid_char, c, pos)
            }
        })
    }
}

#[test]
fn test_lexer() {
    let mut lexer = Lexer::new("1 + 2 -\0  3\n0".to_string()).peekable();

    let tokens = vec![
        tok!(new_int, 1, Pos::new(0, 1, 1)),
        tok!(new_symbol, Symbol::Plus, Pos::new(2, 1, 3)),
        tok!(new_int, 2, Pos::new(4, 1, 5)),
        tok!(new_symbol, Symbol::Minus, Pos::new(6, 1, 7)),
        tok!(new_invalid_char, '\0', Pos::new(7, 1, 8)),
        tok!(new_int, 3, Pos::new(10, 1, 11)),
        tok!(new_int, 0, Pos::new(12, 2, 1)),
        tok!(new_eof, Pos::new(13, 2, 2)),
        tok!(new_eof, Pos::new(13, 2, 2)),
    ];
    for t2 in tokens {
        if let Some(t1p) = lexer.peek() {
            assert_eq!(*t1p, t2);
        }
        assert_eq!(lexer.next(), Some(t2));
    }
}
