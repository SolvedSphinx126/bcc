use lexer::*;
use std::fs::File;
pub mod lexer;

fn main() {
    let test = File::open("./testSrc/return_2.c").unwrap();
    let tokens = lex(test);
    for token in tokens {
        dbg!(token);
    }
}
