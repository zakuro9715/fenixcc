use crate::{tok, Loc, Source, Symbol, Token, TokenKind};
use std::char;
use std::iter::Iterator;

pub struct Lexer<'a> {
    source: &'a Source,
    loc: Loc,
}

#[macro_export]
macro_rules! lex {
    ($code:expr) => (
        Lexer::new(&Source::inline($code))
    );
    ($finemae: expr, $code:expr) => (
        Lexer::new(&Source::new(filename, $code))
    );
}

macro_rules! match_stmt {
    ( $s:stmt ) => { { println!("stmt: {}", stringify!($s)) } };
}

//lexer!(fn eog(&self) -> bool {});
impl<'a> Lexer<'a> {
    pub fn new(source: &'a Source) -> Self {
        Self {
            loc: Loc::head(),
            source,
        }
    }

    fn eof(&self) -> bool {
        self.loc.offset >= self.source.code.len()
    }

    fn peek_char(&mut self) -> char {
        if let Some(c) = self.source.code.get(self.loc.offset) {
            *c
        } else {
            char::REPLACEMENT_CHARACTER
        }
    }

    fn skip_whitespaces(&mut self) {
        let _ = self.read_while(|c| c.is_whitespace());
    }

    fn read_while<F: Fn(char) -> bool>(&mut self, f: F) -> String {
        let start_loc = self.loc;
        while !self.eof() && f(self.peek_char()) {
            self.consume()
        }
        self.source.code[start_loc.offset..self.loc.offset]
            .iter()
            .collect()
    }

    fn read_integer(&mut self) -> Token {
        let loc = self.loc;
        let int = self
            .read_while(|b| b.is_ascii_digit())
            .parse::<i64>()
            .unwrap();
        Token::new(TokenKind::Int(int), loc)
    }

    fn consume(&mut self) {
        if self.eof() {
            return;
        }
        match self.peek_char() {
            '\n' => {
                self.loc.offset += 1;
                self.loc.line += 1;
                self.loc.col = 1;
            }
            '\r' => {
                self.loc.offset += 1;
                self.loc.line += 1;
                self.loc.col = 1;
                // CRLF
                if self.peek_char() == '\n' {
                    self.loc.offset += 1
                }
            }
            _ => {
                self.loc.offset += 1;
                self.loc.col += 1;
            }
        };
    }
}

impl<'a> Iterator for &Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.skip_whitespaces();

        let loc = self.loc;

        if self.eof() {
            return Some(tok!(new_eof, loc));
        }

        Some(match self.peek_char() {
            '+' => {
                let _ = self.consume();
                tok!(new_symbol, Symbol::Plus, loc)
            }
            '-' => {
                let _ = self.consume();
                tok!(new_symbol, Symbol::Minus, loc)
            }
            c if c.is_ascii_digit() => self.read_integer(),
            c => {
                let _ = self.consume();
                tok!(new_invalid_char, c, loc)
            }
        })
    }
}

#[test]
fn test_lexer() {
    let s = Source::new("", "1 + 2 -\0  3\n0");
    let mut lexer = Lexer::new(&s).peekable();

    let tokens = vec![
        tok!(new_int, 1, Loc::new(0, 1, 1)),
        tok!(new_symbol, Symbol::Plus, Loc::new(2, 1, 3)),
        tok!(new_int, 2, Loc::new(4, 1, 5)),
        tok!(new_symbol, Symbol::Minus, Loc::new(6, 1, 7)),
        tok!(new_invalid_char, '\0', Loc::new(7, 1, 8)),
        tok!(new_int, 3, Loc::new(10, 1, 11)),
        tok!(new_int, 0, Loc::new(12, 2, 1)),
        tok!(new_eof, Loc::new(13, 2, 2)),
        tok!(new_eof, Loc::new(13, 2, 2))
    ];

    tokens.iter().for_each(|t, _| assert_eq(t1, lexer.next()));
    assert_eq(None(), lexer.next())
}
