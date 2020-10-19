use crate::{Token, TokenKind, sym};
#[cfg(test)]
use crate::{tok, Loc};

#[derive(Debug, Clone)]
pub struct AST {
    pub token: Token,
    pub node: Node,
}

#[derive(Debug, Clone)]
pub enum Node {
    BinaryExpr(Box<AST>, Box<AST>),
    IntLiteral(i64),
}

impl AST {
    pub fn new(token: Token, node: Node) -> Self {
        Self { token, node }
    }

    pub fn new_binary_expr(lhs: AST, op: Token, rhs: AST) -> Self {
        Self { token: op, node: Node::BinaryExpr(Box::new(lhs), Box::new(rhs)) }
    }

    pub fn new_literal(token: Token) -> AST {
        let kind = token.kind.clone();
        Self {
            token,
            node: match kind {
                TokenKind::Int(i) =>
                    Node::IntLiteral(i),
                _ => panic!("Invalid token")
            }
        }
    }
}
#[macro_export]
macro_rules! ast {
    ($method:ident, $($args:expr),* $(,)?) => (
        $crate::AST::$method($($args),*)
    );
}

#[test]
fn test_new_literal() {
    AST::new_literal(Token::new(TokenKind::Int(0), Loc::head()));
}

#[test]
#[should_panic]
fn test_new_literal_invalid() {
    AST::new_literal(Token::new(TokenKind::EOF, Loc::head()));
}
