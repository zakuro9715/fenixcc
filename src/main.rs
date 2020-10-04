extern crate getopts;
use getopts::Options;
use std::env;

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
        return;
    }

    if !matches.free.is_empty() {
        if matches.free.len() > 1 {
            print_help(&program, opts);
            return
        }
        println!("{}", matches.free[0].clone());
    } else {
        print_help(&program, opts);
        return
    };
    /*
    println!(".intel_syntax noprefix");
    println!("global main");
    println!("main:");
    println!("   mov rax, {}", atoi(argv[1])

*/

}
