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
        Pos {
            offset: offset,
            line: line,
            col: col,
        }
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
        Token {
            text: text,
            kind: kind,
            pos: pos,
        }
    }
}
