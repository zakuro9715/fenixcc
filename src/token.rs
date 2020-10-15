#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Symbol {
    Plus,
    Minus,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TokenError {
    UnexpectedChar {
        actual: Option<char>,
        expected: Option<char>,
    },
    Message(String),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TokenKind {
    Error(TokenError),
    Symbol(Symbol),
    Int(i64),
    EOF,
}

impl TokenKind {
    pub fn is_literal(&self) -> bool {
        match self {
            TokenKind::Int(_) => true,
            _ => false,
        }
    }
}

#[test]
fn test_token_kind_is_literal() {
    assert!(!TokenKind::Error(TokenError::Message("".to_string())).is_literal());
    assert!(!TokenKind::Symbol(Symbol::Plus).is_literal());
    assert!(TokenKind::Int(0).is_literal());
    assert!(!TokenKind::EOF.is_literal());
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Pos {
    pub offset: usize,
    pub line: usize,
    pub col: usize,
}

impl Pos {
    pub fn head() -> Pos {
        Self {
            offset: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn new(offset: usize, line: usize, col: usize) -> Pos {
        Self { offset, line, col }
    }
}

#[test]
fn test_pos_new() {
    let pos = Pos::new(1, 2, 3);
    assert_eq!(pos.offset, 1);
    assert_eq!(pos.line, 2);
    assert_eq!(pos.col, 3);
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: Pos,
}

impl Token {
    pub fn new(kind: TokenKind, pos: Pos) -> Self {
        Self { kind, pos }
    }

    pub fn new_symbol(sym: Symbol, pos: Pos) -> Self {
        Self::new(TokenKind::Symbol(sym), pos)
    }

    pub fn new_int(v: i64, pos: Pos) -> Self {
        Self::new(TokenKind::Int(v), pos)
    }

    pub fn new_error(err: TokenError, pos: Pos) -> Self {
        Self::new(TokenKind::Error(err), pos)
    }

    pub fn new_unexpected_char(expected: Option<char>, actual: Option<char>, pos: Pos) -> Self {
        Self::new_error(TokenError::UnexpectedChar { expected, actual }, pos)
    }

    pub fn new_invalid_char(c: char, pos: Pos) -> Self {
        Self::new_unexpected_char(None, Some(c), pos)
    }

    pub fn new_eof(pos: Pos) -> Self {
        Self::new(TokenKind::EOF, pos)
    }
}

#[macro_export]
macro_rules! tok {
    ($method:ident $(,$args:expr)*) => (
        Token::$method($($args),*)
    );
}

#[test]
fn test_token_new() {
    let tok = Token::new(TokenKind::Int(1), Pos::head());
    assert_eq!(tok.kind, TokenKind::Int(1));
    assert_eq!(tok.pos, Pos::head());
}
