#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Symbol {
    Plus,
    Minus,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenKind {
    Error,
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
    assert!(!TokenKind::Error.is_literal());
    assert!(!TokenKind::Symbol(Symbol::Plus).is_literal());
    assert!(TokenKind::Int(0).is_literal());
    assert!(!TokenKind::EOF.is_literal());
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Pos {
    pub offset: usize,
    pub line: usize,
    pub col: usize,
}

impl Pos {
    pub fn head() -> Pos {
        Pos {
            offset: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn new(offset: usize, line: usize, col: usize) -> Pos {
        Pos { offset, line, col }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub text: String,
    pub kind: TokenKind,
    pub pos: Pos,
}

impl Token {
    pub fn new(text: String, kind: TokenKind, pos: Pos) -> Token {
        Token { text, kind, pos }
    }
}
