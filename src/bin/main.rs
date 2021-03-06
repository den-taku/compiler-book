#![allow(dead_code)]
use std::env;
// use std::io::Write;
use std::process;
// use std::process::Command;
use compiler_book::error::*;
use compiler_book::generator::*;
use compiler_book::lexer::*;
use compiler_book::parser::*;
use compiler_book::static_check::*;

// read compiler book until step10: local variable
fn main() {
    // read program
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("invalid number of arguments.");
        process::exit(1);
    }

    // tokenize program
    let token_stream = TokenStream::tokenize(args[1].clone());
    match token_stream {
        Ok(_) => {}
        Err((message, byte)) => {
            error_at(args[1].clone(), byte, message);
            panic!()
        }
    }

    // enforce static check to tokinized stream
    if let Err((message, position)) = verify_stream(&token_stream.clone().unwrap()) {
        error_position(position, &token_stream.unwrap(), args[1].clone(), message);
        panic!()
    }

    // parse stream
    let ast = parser(&mut token_stream.clone().unwrap());
    if let Err((message, position)) = ast {
        error_position(position, &token_stream.unwrap(), args[1].clone(), message);
        panic!()
    }

    // generate assembly program
    let program = generate_program03(&ast.unwrap());

    println!("{}", program);
}
