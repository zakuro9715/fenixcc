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

#[derive(PartialEq, Debug, Clone)]
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

    pub fn new_int(v: i64, loc: Loc) -> Self {
        Self::new(TokenKind::Int(v), loc)
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

#[macro_export]
macro_rules! tok {
    ($method:ident $(,$args:expr)* $(,)?) => (
        $crate::Token::$method($($args),*)
    );
}

#[cfg(test)]
#[macro_export]
macro_rules! head_tok {
    ($method:ident $(,$args:expr)* $(,)?) => {
        $crate::Token::$method($($args),*, $crate::Loc::head())
    }
}

#[macro_export]
macro_rules! sym {
    ($sym:ident) => ($crate::TokenKind::Symbol($crate::Symbol::$sym));
}

#[test]
fn test_token_new() {
    let tok = Token::new(TokenKind::Int(1), Loc::head());
    assert_eq!(tok.kind, TokenKind::Int(1));
    assert_eq!(tok.loc, Loc::head());
}
