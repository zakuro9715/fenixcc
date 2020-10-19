extern crate getopts;

use getopts::Options;
use std::env;
use std::process::exit;
use std::fs;

use fenixcc::{Lexer, Parser, Source};

fn print_help(program: &str, opts: Options) {
    let brief = format!("Usage: {} INPUT [options]", program);
    print!("{}", opts.usage(&brief));
}

fn setup_opts(opts: &mut Options) {
    opts.optflag("h", "help", "Print help");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    setup_opts(&mut opts);

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_help(&program, opts);
        exit(0);
    }

    let source = if !matches.free.is_empty() {
        if matches.free.len() > 1 {
            print_help(&program, opts);
            exit(1);
        }
        let filename = matches.free[0].clone();
        let code = fs::read_to_string(&filename).unwrap();
        Source::new(filename, code)
    } else {
        print_help(&program, opts);
        exit(0);
    };

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    match parser.parse() {
        Ok(ast) => println!("{:#?}", ast),
        Err(err) => println!("Error: {:#?}", err),
    }
}
