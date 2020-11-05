use crate::{sym, Token, TokenKind};
#[cfg(test)]
use crate::{tok, head_tok, Loc};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct AST {
    pub token: Token,
    pub node: Node,
}

#[macro_export]
macro_rules! ast {
    ($method:ident, $($args:expr),* $(,)?) => (
        $crate::AST::$method($($args),*)
        );
}

#[cfg(test)]
macro_rules! ast_zero_literal {
    () => {
        ast!(new_literal, $crate::tok!(new_int, 0, $crate::Loc::head()))
    };
}


#[derive(Eq, Debug, Clone)]
pub enum Node {
    Addition(Box<AST>, Box<AST>),
    Subtraction(Box<AST>, Box<AST>),
    IntLiteral(i64),
}


impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        use Node::*;
        macro_rules! eq1 {
            ($node:ident)=> {
                if let ($node(lhs), $node(rhs)) = (self, other) {
                    *lhs == *rhs
                } else {
                    false
                }
            }
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
}


impl AST {
    pub fn new(token: Token, node: Node) -> Self {
        Self {
            token,
            node,
        }
    }

    pub fn new_binary_expr(lhs: AST, op: Token, rhs: AST) -> Self {
        let kind = op.kind.clone();
        Self::new(
            op,
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
            token,
            match kind {
                TokenKind::Int(i) => Node::IntLiteral(i),
                _ => panic!("Invalid token"),
            },
        )
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
            lhs.clone(),
            tok!(new, sym!(Minus), Loc::head()),
            rhs.clone(),
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

pub trait Visitor<R> {
    fn visit(&mut self, ast: &AST) -> R {
        match &ast.node {
            Node::IntLiteral(i) => self.visit_int_literal(*i),
            Node::Addition(lhs, rhs) => {
                let l = self.visit(&lhs);
                let r = self.visit(&rhs);
                self.visit_addition(l, r)
            }
            Node::Subtraction(lhs, rhs) => {
                let l = self.visit(&lhs);
                let r = self.visit(&rhs);
                self.visit_subtraction(l, r)
            }
        }
    }
    fn visit_int_literal(&mut self, i: i64) -> R;
    fn visit_addition(&mut self, lhs: R, rhs: R) -> R;
    fn visit_subtraction(&mut self, lhs: R, rhs: R) -> R;
}

mod ir {
    use super::{Visitor, AST};
    use crate::Instruction::*;
    use crate::IR;

    impl From<AST> for IR {
        fn from(ast: AST) -> IR {
            IRTranslator::new().translate(&ast)
        }
    }

    pub struct IRTranslator {
        buffer: IR,
    }

    impl IRTranslator {
        pub fn new() -> Self {
            Self {
                buffer: IR::new(),
            }
        }

        pub fn translate(&mut self, ast: &AST) -> IR {
            self.visit(ast);
            self.take()
        }

        pub fn take(&mut self) -> IR {
            std::mem::take(&mut self.buffer)
        }
    }

    impl Visitor<()> for IRTranslator {
        fn visit_int_literal(&mut self, i: i64) -> () {
            self.buffer.push(PushI(i));
        }
        fn visit_addition(&mut self, _: (), _: ()) {
            self.buffer.push(AddI);
            ()
        }
        fn visit_subtraction(&mut self, _: (), _: ()) {
            self.buffer.push(SubI);
            ()
        }
    }

    #[cfg(test)]
    mod translater_tests {
        use super::IRTranslator;
        use crate::Instruction::*;
        use crate::{head_tok, sym};

        #[test]
        fn test_translate() {
            let mut t = IRTranslator::new();
            let ir = t.translate(&ast!(
                new_binary_expr,
                ast!(
                    new_binary_expr,
                    ast!(new_literal, head_tok!(new_int, 1)),
                    head_tok!(new, sym!(Plus)),
                    ast!(new_literal, head_tok!(new_int, 2)),
                ),
                head_tok!(new, sym!(Minus)),
                ast!(new_literal, head_tok!(new_int, -3)),
            ));
            assert_eq!(ir, vec![PushI(1), PushI(2), AddI, PushI(-3), SubI]);
            assert_eq!(t.buffer, vec![]);
        }

        #[test]
        fn test_take() {
            let mut t = IRTranslator::new();
            t.buffer = vec![PopI];
            assert_eq!(t.take(), vec![PopI]);
            assert_eq!(t.buffer, vec![]);
        }
    }
}

pub use ir::IRTranslator;
