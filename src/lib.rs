pub use token::*;
pub mod ast;
pub use ast::*;
pub mod translate;
pub use translate::*;

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

pub fn compile<'a>(filename: impl Into<String>) -> parser::Result<String> {
    use std::fs;
    let filename_string: String = filename.into();
    let code = fs::read_to_string(&filename_string).unwrap();
    let source = &Source::new(filename_string, code);
    let ast = Parser::new(lexer::Lexer::new(source)).parse()?;
    let ir: IR = ast.into();
    Ok(x86_64::compile(&ir))
}
