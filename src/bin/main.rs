#![allow(dead_code)]
use std::env;
// use std::io::Write;
use std::process;
// use std::process::Command;
use compiler_book::error::*;
use compiler_book::generator::*;
use compiler_book::lexer::*;
use compiler_book::parser::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("invalid number of arguments.");
        process::exit(1);
    }

    let token_stream = TokenStream::tokenize01(args[1].clone());
    match token_stream {
        Ok(_) => {}
        Err((message, byte)) => {
            error_at(args[1].clone(), byte, message);
            panic!()
        }
    }

    let ast = parser(&mut token_stream.clone().unwrap());

    if let Err((message, position)) = ast {
        error_position(position, &token_stream.unwrap(), args[1].clone(), message);
        panic!()
    }

    let program = generate_program02(&ast.unwrap());

    println!("{}", program);
}
