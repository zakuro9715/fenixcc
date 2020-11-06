use crate::{sym, Token, TokenKind};
#[cfg(test)]
use crate::{ast, ast_zero_literal, tok, head_tok, Loc};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AST {
    pub token: Option<Token>,
    pub node: Node,
}


macro_rules! define_node {
    ($($name:ident,)*) => {
        #[derive(PartialEq, Eq, Debug, Clone)]
        pub enum Node {
            $(
                $name(nodes::$name)
            ),*
        }
    };
}

define_node!{
    Block,
    ExprStatement,
    Addition,
    Subtraction,
    IntLiteral,
}

pub mod nodes {
    use super::AST;
    macro_rules! binary {
        ($name:ident) => {
            #[derive(Eq, Debug, Clone)]
            pub struct $name {
                pub lhs: Box<AST>,
                pub rhs: Box<AST>,
            }
            impl PartialEq for $name {
                fn eq(&self, other: &Self) -> bool {
                    *self.lhs == *other.lhs && *self.rhs ==*other.rhs
                }
            }
        };
    }
    macro_rules! value {
        ($name:ident, $type:ty) => {
            #[derive(PartialEq, Eq, Debug, Clone)]
            pub struct $name {
                pub value: $type,
            }
        };
    }

    #[derive(PartialEq, Eq, Debug, Clone)]
    pub struct Block {
        pub items: Vec<AST>
    }
    #[derive(PartialEq, Eq, Debug, Clone)]
    pub struct ExprStatement {
        pub expr: Box<AST>,
    }

    binary!{Addition}
    binary!{Subtraction}

    value!{IntLiteral, i64}

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::{ast, ast_zero_literal, head_tok};

        #[test]
        fn test_binary_eq() {
            assert_eq!(
                Addition{
                    lhs: Box::new(ast_zero_literal!()),
                    rhs: Box::new(ast_zero_literal!()),
                },
                Addition{
                    lhs: Box::new(ast_zero_literal!()),
                    rhs: Box::new(ast_zero_literal!()),
                },
            );
            assert_ne!(
                Addition{
                    lhs: Box::new(ast_zero_literal!()),
                    rhs: Box::new(ast_zero_literal!()),
                },
                Addition{
                    lhs: Box::new(ast_zero_literal!()),
                    rhs: Box::new(ast!(new_literal, head_tok!(new_int, 1))),
                },
            );
        }
    }
}

use nodes::*;


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
                sym!(Plus) => Node::Addition(nodes::Addition{
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }),
                sym!(Minus) => Node::Subtraction(nodes::Subtraction{
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }),
                _ => panic!("Invalid token"),
            },
        )
    }

    pub fn new_literal(token: Token) -> AST {
        let kind = token.kind.clone();
        Self::new(
            Some(token),
            match kind {
                TokenKind::Int(value) => Node::IntLiteral(nodes::IntLiteral{
                    value
                }),
                _ => panic!("Invalid token"),
            },
        )
    }

    pub fn new_block(items: Vec<AST>) -> Self {
       Self::new(None, Node::Block(nodes::Block{ items}))
    }

    pub fn new_expr_statement(expr :AST) -> Self {
        Self::new(None, Node::ExprStatement(nodes::ExprStatement{
            expr: Box::new(expr)
        }))
    }
}


#[cfg(test)]
mod tests {
    use crate::{AST, Token, TokenKind, Loc, tok, ast_zero_literal};

    #[test]
    #[should_panic]
    fn test_new_literal_invalid() {
        AST::new_literal(Token::new(TokenKind::EOF, Loc::head()));
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
}


pub trait Visitor<R: Default, E> {
    fn visit(&mut self, ast: &AST) -> Result<R, E> {
        match &ast.node {
            Node::Block(block) => {
                for v in &block.items {
                    self.visit(v)?;
                }
                Ok(Default::default())
            }
            Node::ExprStatement(expr) => {
                self.visit_expr_statement_left()?;
                let v = self.visit(expr.expr.as_ref())?;
                self.visit_expr_statement_right(v)
            }
            Node::IntLiteral(lit) => self.visit_int_literal(lit),
            Node::Addition(v) => {
                let l = self.visit(&v.lhs)?;
                let r = self.visit(&v.rhs)?;
                self.visit_addition(l, r)
            }
            Node::Subtraction(v) => {
                let l = self.visit(&v.lhs)?;
                let r = self.visit(&v.rhs)?;
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
    fn visit_int_literal(&mut self, _i: &IntLiteral) -> Result<R, E> {
        Ok(Default::default())
    }
    fn visit_addition(&mut self, _lhs: R, _rhs: R) -> Result<R, E> {
        Ok(Default::default())
    }
    fn visit_subtraction(&mut self, _lhs: R, _rhs: R) -> Result<R, E> {
        Ok(Default::default())
    }
}
