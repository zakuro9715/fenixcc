#[rustfmt::skip::macros(format)]
pub mod x86_64 {
    use crate::Instruction::*;
    use crate::{Instruction, IR};

    fn compile_binary_operation(op: &str) -> String {
        format!("\
            \tpop rdi\n\
            \tpop rax\n\
            \t{} rax, rdi\n\
            \tpush rax\n\
        ", op)
    }

    fn compile_instruction(inst: &Instruction) -> String {
        match inst {
            PushI(i) => format!("\tpush {}\n", i),
            PopI => "\tpop rax\n".to_string(),
            AddI => compile_binary_operation("add"),
            SubI => compile_binary_operation("sub"),
        }
    }

    pub fn compile(ir: &IR) -> String {
        let body = ir
            .iter()
            .fold(String::new(), |s, inst| s + &compile_instruction(&inst));
        format!("\
            .intel_syntax noprefix\n\
            .global main\n\
            main: \n\
            {}\n\
            \tret\n\
        ", body)
    }

    #[test]
    fn test_pushi() {
        let s = compile_instruction(&PushI(9));
        assert_eq!(1, s.lines().count());
        assert!(s.contains("push"));
        assert!(s.contains('9'));
    }

    #[test]
    fn test_popi() {
        let s = compile_instruction(&PopI);
        assert_eq!(1, s.lines().count());
        assert!(s.contains("pop"));
    }

    #[test]
    fn test_addi() {
        let s = compile_instruction(&AddI);
        assert_eq!(4, s.lines().count());
        assert!(s.contains("push"));
        assert!(s.contains("add"));
    }
    #[test]
    fn test_subi() {
        let s = compile_instruction(&SubI);
        assert_eq!(4, s.lines().count());
        assert!(s.contains("push"));
        assert!(s.contains("sub"));
    }
}
