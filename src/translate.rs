use crate::{IR, Visitor, AST};
use crate::Instruction::*;

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
        self.visit(ast).unwrap();
        self.take()
    }

    pub fn take(&mut self) -> IR {
        std::mem::take(&mut self.buffer)
    }
}

impl Visitor<(), ()> for IRTranslator {
    fn visit_expr_statement_right(&mut self, _: ()) -> Result<(), ()> {
        self.buffer.push(PopI);
        Ok(())
    }
    fn visit_int_literal(&mut self, i: i64) -> Result<(), ()> {
        self.buffer.push(PushI(i));
        Ok(())
    }
    fn visit_addition(&mut self, _: (), _: ()) -> Result<(), ()>{
        self.buffer.push(AddI);
        Ok(())
    }
    fn visit_subtraction(&mut self, _: (), _: ()) -> Result<(), ()> {
        self.buffer.push(SubI);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::IRTranslator;
    use crate::Instruction::*;
    use crate::{ast, head_tok, sym};

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
