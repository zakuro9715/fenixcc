extern crate getopts;

use getopts::Options;
use std::env;
use std::process::exit;

use fenixcc::compile;

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

    let filename = if !matches.free.is_empty() {
        if matches.free.len() > 1 {
            print_help(&program, opts);
            exit(1);
        }
        matches.free[0].clone()
    } else {
        print_help(&program, opts);
        exit(0);
    };

    match compile(filename) {
        Ok(asm) => println!("{}", asm), // generate_code(&ast)),
        Err(err) => println!("Error: {:#?}", err),
    }
}
