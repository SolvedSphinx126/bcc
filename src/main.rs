use generator::generate;
use lexer::*;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

use parser::parse;
pub mod generator;
pub mod lexer;
pub mod parser;

fn main() {
    let src_file: Option<String> = env::args().nth(1);
    let src_file = match src_file {
        Some(file) => file,
        None => panic!("You need to supply a file to the compiler"),
    };
    let input_src = File::open(&src_file).unwrap();

    let basename = Path::new(&src_file).with_extension("");
    let output_asm = &format!("{}.s", basename.to_str().unwrap());
    let output_exe = &basename.to_str().unwrap();

    let tokens = lex(input_src);

    let pgrm = parse(tokens).unwrap();

    let mut asm = generate(pgrm);

    let mut output_asm_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(Path::new(output_asm))
        .unwrap();

    for line in asm.iter_mut() {
        line.push('\n');
        let _ = output_asm_file.write(line.as_bytes());
    }

    let mut binding = Command::new("gcc");
    let assemble_command = binding
        .arg("-m64")
        .arg(output_asm)
        .arg("-o")
        .arg(output_exe);
    let mut path = env::args().next().unwrap();
    path.push('/');
    assemble_command.current_dir(std::env::current_dir().unwrap());
    let _res = assemble_command.output().expect("Assembly failed");
}
