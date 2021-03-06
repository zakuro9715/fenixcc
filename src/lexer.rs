use crate::{tok, Loc, Source, Symbol, Token, TokenKind};
use std::char;
use std::iter::Iterator;


pub struct Lexer<'a> {
    source: &'a Source,
    loc: Loc,
}

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

    fn peek_char(&self) -> char {
        if let Some(c) = self.source.code.get(self.loc.offset) {
            *c
        } else {
            char::REPLACEMENT_CHARACTER
        }
    }

    fn skip_whitespaces(&mut self) {
        let _ = self.read_while(|c| c.is_whitespace());
    }
}


fn is_ident_first_char(c :char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident_char(c :char) -> bool {
    is_ident_first_char(c) || c.is_ascii_digit()
}

impl<'a> Lexer<'a> {
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

    fn read_ident(&mut self) -> Token {
        let loc = self.loc;
        let first = self.peek_char();
        self.consume();
        tok!(new_ident, first.to_string() + &self.read_while(is_ident_char), loc)
    }

    fn consume(&mut self) {
        if self.eof() {
            panic!("Cannot consume after eof");
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

    fn consume_and(&mut self, t: Token) -> Token {
        self.consume_n_and(1, t)
    }

    fn consume_n_and(&mut self, n: usize, t: Token) -> Token {
        for _ in 0..n {
            self.consume()
        }
        t
    }
}


impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.skip_whitespaces();

        let loc = self.loc;

        if self.eof() {
            return Some(tok!(new_eof, loc));
        }

        macro_rules! read_sym {
            ($sym:ident; $n:expr) => {
                self.consume_n_and($n, tok!(new_symbol, Symbol::$sym, loc))
            };
        }
        macro_rules! read_sym1 {
            ($sym:ident) => { read_sym!($sym; 1) };
        }
        Some(match self.peek_char() {
            '+' => read_sym1!(Plus),
            '-' => read_sym1!(Minus),
            '*' => read_sym1!(Asterisk),
            '/' => read_sym1!(Slash),
            ';' => read_sym1!(Semicolon),
            '=' => read_sym1!(Equal),
            c if is_ident_first_char(c) => self.read_ident(),
            c if c.is_ascii_digit() => self.read_integer(),
            c => self.consume_and(tok!(new_invalid_char, c, loc)),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{tok, head_tok, sym, Source, Lexer, Symbol, Token, Loc};

    fn test_lex(code :&str, expected :Vec<Token>) {
        let s = Source::new("", code);
        let mut lexer = Lexer::new(&s).peekable();
        for t in expected.into_iter() {
            assert_eq!(Some(t), lexer.next());
        }
    }

    #[test]
    fn test_simple() {
        test_lex("1 + 11 -\0  \n3", vec![
            tok!(new_int, 1, Loc::new(0, 1, 1)),
            tok!(new_symbol, Symbol::Plus, Loc::new(2, 1, 3)),
            tok!(new_int, 11, Loc::new(4, 1, 5)),
            tok!(new_symbol, Symbol::Minus, Loc::new(7, 1, 8)),
            tok!(new_invalid_char, '\0', Loc::new(8, 1, 9)),
            tok!(new_int, 3, Loc::new(12, 2, 1)),
            tok!(new_eof, Loc::new(13, 2, 2)),
            tok!(new_eof, Loc::new(13, 2, 2)),
        ]);
    }

    #[test]
    fn test_symbols() {
        test_lex("+-*/;=", vec![
            tok!(new, sym!(Plus), Loc::new(0, 1, 1)),
            tok!(new, sym!(Minus), Loc::new(1, 1, 2)),
            tok!(new, sym!(Asterisk), Loc::new(2, 1, 3)),
            tok!(new, sym!(Slash), Loc::new(3, 1, 4)),
            tok!(new, sym!(Semicolon), Loc::new(4, 1, 5)),
            tok!(new, sym!(Equal), Loc::new(5, 1, 6)),
            tok!(new_eof, Loc::new(6, 1, 7)),
        ])
    }

    #[test]
    fn test_newline() {
        test_lex("0\r0\r\n0\n\n\r0", vec![
            tok!(new_int, 0, Loc::new(0, 1, 1)),
            tok!(new_int, 0, Loc::new(2, 2, 1)),
            tok!(new_int, 0, Loc::new(5, 3, 1)),
            tok!(new_int, 0, Loc::new(9, 6, 1)),
            tok!(new_eof, Loc::new(10, 6, 2)),
        ])
    }

    #[test]
    fn test_ident() {
        test_lex("_abc", vec![
            head_tok!(new_ident, "_abc"),
       ])
    }
}
