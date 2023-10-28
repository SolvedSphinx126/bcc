use lexer::*;
use std::env;
use std::fs::File;

use parser::parse;
pub mod lexer;
pub mod parser;

fn main() {
    let src_file: Option<String> = env::args().nth(1);
    let src_file = match src_file {
        Some(file) => file,
        None => panic!("You need to supply a file to the compiler")
    };
    let test = File::open(src_file).unwrap();
    let tokens = lex(test);
    if let Ok(pgrm) = parse(tokens) {
        let ret_val = pgrm.fns.statements.result.value;
        let fn_name= &pgrm.fns.name.value;
        println!("{} returns {}", fn_name, ret_val);
        dbg!(pgrm);
        std::process::exit(ret_val.try_into().unwrap());
    }
    else {
        println!("There was a syntax error somewhere, good luck finding it!")
    }

    std::process::exit(0);
}
