use crate::token::Token;
#[cfg(test)]
use crate::token::{Pos, TokenKind};

#[derive(Debug, Clone)]
pub struct AST {
    token: Token,
    node: Node,
}

#[derive(Debug, Clone)]
pub enum Node {
    BinaryExpr(Box<AST>, Box<AST>),
    Literal,
}

impl AST {
    pub fn new(token: Token, node: Node) -> Self {
        Self { token, node }
    }

    pub fn new_binary_expr(lhs: AST, op: Token, rhs: AST) -> Self {
        Self { token: op, node: Node::BinaryExpr(Box::new(lhs), Box::new(rhs)) }
    }

    pub fn new_literal(token: Token) -> AST {
        debug_assert!(token.kind.is_literal());
        Self {
            token,
            node: Node::Literal,
        }
    }
}
#[macro_export]
macro_rules! ast {
    ($method:ident, $($args:expr),* $(,)?) => (
        AST::$method($($args),*)
    );
}

#[test]
fn test_new_literal() {
    AST::new_literal(Token::new(TokenKind::Int(0), Pos::head()));
}

#[test]
#[should_panic]
fn test_new_literal_invalid() {
    AST::new_literal(Token::new(TokenKind::EOF, Pos::head()));
}
