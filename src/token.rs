#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Loc {
    pub offset: usize,
    pub line: usize,
    pub col: usize,
}

impl Loc {
    pub fn head() -> Loc {
        Self {
            offset: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn new(offset: usize, line: usize, col: usize) -> Loc {
        Self { offset, line, col }
    }
}

#[test]
fn test_loc_new() {
    let loc = Loc::new(1, 2, 3);
    assert_eq!(loc.offset, 1);
    assert_eq!(loc.line, 2);
    assert_eq!(loc.col, 3);
}


#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Symbol {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Semicolon,
    Equal,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Keyword {
    Return,
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
    Keyword(Keyword),
    Ident(String),
    Int(i64),
    EOF,
}

impl TokenKind {
    pub fn is_literal(&self) -> bool {
        matches!(self, TokenKind::Int(_))
    }
}

#[test]
fn test_token_kind_is_literal() {
    assert!(!TokenKind::Error(TokenError::Message("".to_string())).is_literal());
    assert!(!TokenKind::Symbol(Symbol::Plus).is_literal());
    assert!(TokenKind::Int(0).is_literal());
    assert!(!TokenKind::EOF.is_literal());
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub loc: Loc,
}

impl Token {
    pub fn new(kind: TokenKind, loc: Loc) -> Self {
        Self { kind, loc }
    }

    pub fn new_symbol(sym: Symbol, loc: Loc) -> Self {
        Self::new(TokenKind::Symbol(sym), loc)
    }

    pub fn new_keyword(keyword: Keyword, loc: Loc) -> Self {
        Self::new(TokenKind::Keyword(keyword), loc)
    }

    pub fn new_int(v: i64, loc: Loc) -> Self {
        Self::new(TokenKind::Int(v), loc)
    }

    pub fn new_ident(s: impl Into<String>, loc: Loc) -> Self {
        Self::new(TokenKind::Ident(s.into()), loc)
    }

    pub fn new_error(err: TokenError, loc: Loc) -> Self {
        Self::new(TokenKind::Error(err), loc)
    }

    pub fn new_unexpected_char(expected: Option<char>, actual: Option<char>, loc: Loc) -> Self {
        Self::new_error(TokenError::UnexpectedChar { expected, actual }, loc)
    }

    pub fn new_invalid_char(c: char, loc: Loc) -> Self {
        Self::new_unexpected_char(None, Some(c), loc)
    }

    pub fn new_eof(loc: Loc) -> Self {
        Self::new(TokenKind::EOF, loc)
    }
}

#[test]
fn test_token_new() {
    let tok = Token::new(TokenKind::Int(1), Loc::head());
    assert_eq!(tok.kind, TokenKind::Int(1));
    assert_eq!(tok.loc, Loc::head());
}
