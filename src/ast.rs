use crate::{sym, Token, TokenKind};
#[cfg(test)]
use crate::{ast, ast_zero_literal, tok, head_tok, Loc};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AST {
    pub token: Option<Token>,
    pub node: Node,
}

#[derive(Eq, Debug, Clone)]
pub enum Node {
    Block(Vec<AST>),
    ExprStatement(Box<AST>),
    Addition(Box<AST>, Box<AST>),
    Subtraction(Box<AST>, Box<AST>),
    IntLiteral(i64),
}


impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        use Node::*;
        macro_rules! eq1 {
            ($node:ident) => {
                if let ($node(lhs), $node(rhs)) = (self, other) {
                    *lhs == *rhs
                } else {
                    false
                }
            };
        }
        macro_rules! eq2 {
            ($node:ident) => {
                if let ($node(l1, r1), $node(l2, r2)) = (self, other) {
                    *l1 == *l2 && *r1 == *r2
                } else {
                    false
                }
            };
        }
        match self {
            Addition(_, _) => eq2!(Addition),
            Subtraction(_, _) => eq2!(Subtraction),
            IntLiteral(_) => eq1!(IntLiteral),
            Block(_) => eq1!(Block),
            ExprStatement(_) => eq1!(ExprStatement),
        }
    }
}
#[test]
fn test_node_eq() {
    use Node::*;
    assert_eq!(
        Addition(
            Box::new(ast!(new_literal, head_tok!(new_int, 0))),
            Box::new(ast!(new_literal, head_tok!(new_int, 0))),
        ),
        Addition(
            Box::new(ast!(new_literal, head_tok!(new_int, 0))),
            Box::new(ast!(new_literal, head_tok!(new_int, 0))),
        ),
    );
    assert_ne!(
        Addition(
            Box::new(ast!(new_literal, head_tok!(new_int, 0))),
            Box::new(ast!(new_literal, head_tok!(new_int, 0))),
        ),
        Addition(
            Box::new(ast!(new_literal, head_tok!(new_int, 0))),
            Box::new(ast!(new_literal, head_tok!(new_int, 1))),
        ),
    );
    assert_eq!(
        Subtraction(
            Box::new(ast!(new_literal, head_tok!(new_int, 0))),
            Box::new(ast!(new_literal, head_tok!(new_int, 0))),
        ),
        Subtraction(
            Box::new(ast!(new_literal, head_tok!(new_int, 0))),
            Box::new(ast!(new_literal, head_tok!(new_int, 0))),
        ),
    );
    assert_ne!(
        Subtraction(
            Box::new(ast!(new_literal, head_tok!(new_int, 0))),
            Box::new(ast!(new_literal, head_tok!(new_int, 0))),
        ),
        Subtraction(
            Box::new(ast!(new_literal, head_tok!(new_int, 0))),
            Box::new(ast!(new_literal, head_tok!(new_int, 1))),
        ),
    );

    assert_ne!(
        Addition(
            Box::new(ast!(new_literal, head_tok!(new_int, 0))),
            Box::new(ast!(new_literal, head_tok!(new_int, 0))),
        ),
        Subtraction(
            Box::new(ast!(new_literal, head_tok!(new_int, 0))),
            Box::new(ast!(new_literal, head_tok!(new_int, 0))),
        ),
    );

    assert_eq!(IntLiteral(0), IntLiteral(0));
    assert_ne!(IntLiteral(0), IntLiteral(1));
    assert_eq!(Block(vec![ast_zero_literal!()]), Block(vec![ast_zero_literal!()]));
    assert_ne!(Block(vec![ast_zero_literal!()]), Block(vec![ast!(new_literal, head_tok!(new_int, 1))]));
    assert_eq!(ExprStatement(Box::new(ast_zero_literal!())), ExprStatement(Box::new(ast_zero_literal!())));
    assert_ne!(
        ExprStatement(Box::new(ast_zero_literal!())),
        ExprStatement(Box::new(ast!(new_literal, head_tok!(new_int, 1)))),
    );
}


impl AST {
    pub fn new(token: Option<Token>, node: Node) -> Self {
        Self {
            token,
            node,
        }
    }

    pub fn new_binary_expr(lhs: AST, op: Token, rhs: AST) -> Self {
        let kind = op.kind.clone();
        Self::new(
            Some(op),
            match kind {
                sym!(Plus) => Node::Addition(Box::new(lhs), Box::new(rhs)),
                sym!(Minus) => Node::Subtraction(Box::new(lhs), Box::new(rhs)),
                _ => panic!("Invalid token"),
            },
        )
    }

    pub fn new_literal(token: Token) -> AST {
        let kind = token.kind.clone();
        Self::new(
            Some(token),
            match kind {
                TokenKind::Int(i) => Node::IntLiteral(i),
                _ => panic!("Invalid token"),
            },
        )
    }

    pub fn new_block(items: Vec<AST>) -> Self {
       Self::new(None, Node::Block(items))
    }

    pub fn new_expr_statement(expr :AST) -> Self {
        Self::new(None, Node::ExprStatement(Box::new(expr)))
    }
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

#[test]
fn test_new_binary_expr() {
    let (lhs, rhs) = (ast_zero_literal!(), ast_zero_literal!());
    assert!(matches!(
        AST::new_binary_expr(lhs.clone(), tok!(new, sym!(Plus), Loc::head()), rhs.clone()).node,
        Node::Addition(_, _)
    ));
    assert!(matches!(
        AST::new_binary_expr(
            lhs,
            tok!(new, sym!(Minus), Loc::head()),
            rhs,
        )
        .node,
        Node::Subtraction(_, _)
    ));
}
#[test]
#[should_panic]
fn test_new_binary_expr_invalid() {
    AST::new_binary_expr(
        ast_zero_literal!(),
        tok!(new_int, 0, Loc::head()),
        ast_zero_literal!(),
    );
}


pub trait Visitor<R: Default, E> {
    fn visit(&mut self, ast: &AST) -> Result<R, E> {
        match &ast.node {
            Node::Block(items) => {
                for v in items {
                    self.visit(v)?;
                }
                Ok(Default::default())
            }
            Node::ExprStatement(item) => {
                self.visit_expr_statement_left()?;
                let v = self.visit(item.as_ref())?;
                self.visit_expr_statement_right(v)
            }
            Node::IntLiteral(i) => self.visit_int_literal(*i),
            Node::Addition(lhs, rhs) => {
                let l = self.visit(&lhs)?;
                let r = self.visit(&rhs)?;
                self.visit_addition(l, r)
            }
            Node::Subtraction(lhs, rhs) => {
                let l = self.visit(&lhs)?;
                let r = self.visit(&rhs)?;
                self.visit_subtraction(l, r)
            }
        }
    }
    fn visit_expr_statement_left(&mut self) -> Result<R, E> {
        Ok(Default::default())
    }
    fn visit_expr_statement_right(&mut self, _item: R) -> Result<R, E> {
        Ok(Default::default())
    }
    fn visit_int_literal(&mut self, _i: i64) -> Result<R, E> {
        Ok(Default::default())
    }
    fn visit_addition(&mut self, _lhs: R, _rhs: R) -> Result<R, E> {
        Ok(Default::default())
    }
    fn visit_subtraction(&mut self, _lhs: R, _rhs: R) -> Result<R, E> {
        Ok(Default::default())
    }
}
