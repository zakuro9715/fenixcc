#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Symbol {
    Plus,
    Minus,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
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
}

#[test]
fn test_token_new() {
    let tok = Token::new(TokenKind::Int(1), Pos::head());
    assert_eq!(tok.kind, TokenKind::Int(1));
    assert_eq!(tok.pos, Pos::head());
}
