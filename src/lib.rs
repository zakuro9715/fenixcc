pub use token::*;
pub mod ast;
pub use ast::*;

pub mod codegen;
pub use codegen::*;

pub mod ir;
pub use ir::*;

pub mod lexer;
pub use lexer::*;
pub mod parser;
pub use parser::*;

pub mod token;
pub use token::*;

pub mod source;
pub use source::*;

#[macro_use]
mod macros;

pub fn compile<'a>(filename: String) -> parser::Result<String> {
    use std::fs;
    let code = fs::read_to_string(&filename).unwrap();
    let source = &Source::new(filename, code);
    let ast = Parser::new(lexer::Lexer::new(source)).parse()?;
    let ir = IRTranslator::new().translate(&ast);
    Ok(x86_64::compile(&ir))
}
