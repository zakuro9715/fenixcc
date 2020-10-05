use crate::token::Token;
#[cfg(test)] use crate::token::{TokenKind, Pos};


#[derive(Debug, Clone)]
pub struct AST {
    token: Token,
    node: Node,
}

#[derive(Debug, Clone)]
pub enum Node {
    Addition(Box<AST>, Box<AST>),
    Subtraction(Box<AST>, Box<AST>),
    Literal,
}

impl AST {
    pub fn new(token: Token, node: Node) -> Box<AST> {
        Box::new(Self { token, node })
    }

    pub fn new_literal(token: Token) -> Box<AST> {
        debug_assert!(token.kind.is_literal());
        Box::new(Self { token, node: Node::Literal })
    }
}

#[test]
fn test_new_literal() {
    AST::new_literal(Token::new("0".to_string(), TokenKind::Int(0), Pos::head()));
}

#[test]
#[should_panic]
fn test_new_literal_invalid() {
    AST::new_literal(Token::new("".to_string(), TokenKind::EOF, Pos::head()));
}
