#![allow(dead_code)]
use std::env;
// use std::io::Write;
use std::process;
// use std::process::Command;
use compiler_book::lexer::*;
use compiler_book::parser::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("invalid number of arguments.");
        process::exit(1);
    }

    print!("{}", add_sub(&args[1]));

    let token_sequence = TokenStream::tokenize01(args.into_iter().nth(1).unwrap());
    println!("{:?}", token_sequence);
}
