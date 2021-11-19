use crate::lexer::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Byte(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position(pub usize);

pub fn error_position(position: Position, stream: &TokenStream, program: String, message: String) {
    let Position(index) = position;
    error_at(
        program,
        Byte(
            *stream
                .position
                .iter()
                .nth(index)
                .expect("inner error: out of bound"),
        ),
        message,
    );
}

pub fn error_at(program: String, byte: Byte, message: String) {
    println!("{}", program);
    let Byte(index) = byte;
    println!(
        "{}^ {}",
        (0..index).map(|_| " ").collect::<String>(),
        message
    );
}
