use crate::error::*;
use crate::lexer::Token::*;
use crate::lexer::*;
use std::process;
use Node::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Num(i64),
}

pub fn parser01(stream: &mut TokenStream) -> Result<Node, (String, Position)> {
    verify_stream(stream)?;
    Ok(expr01(stream))
}

pub fn expr01(stream: &mut TokenStream) -> Node {
    let mut node = mul01(stream);
    while let Some(token) = stream.sequence.front() {
        match token {
            Reserved(op) if op == &Operator::Add => {
                stream.sequence.pop_front();
                node = Add(Box::new(node), Box::new(mul01(stream)))
            }
            Reserved(op) if op == &Operator::Sub => {
                stream.sequence.pop_front();
                node = Sub(Box::new(node), Box::new(mul01(stream)))
            }
            Eof => {
                break;
            }
            _ => return node,
        }
    }
    node
}

pub fn mul01(stream: &mut TokenStream) -> Node {
    let mut node = primary01(stream);
    while let Some(token) = stream.sequence.front() {
        match token {
            Reserved(op) if op == &Operator::Mul => {
                stream.sequence.pop_front();
                node = Mul(Box::new(node), Box::new(primary01(stream)))
            }
            Reserved(op) if op == &Operator::Div => {
                stream.sequence.pop_front();
                node = Div(Box::new(node), Box::new(primary01(stream)))
            }
            Eof => {
                break;
            }
            _ => return node,
        }
    }
    node
}

pub fn primary01(stream: &mut TokenStream) -> Node {
    if let Some(token) = stream.sequence.pop_front() {
        match token {
            LeftBra => {
                let node = expr01(stream);
                if let Some(token) = stream.sequence.pop_front() {
                    if token == RightBra {
                        node
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
            }
            Number(number) => Num(number),
            _ => unreachable!(),
        }
    } else {
        unreachable!()
    }
}

fn verify_stream(stream: &TokenStream) -> Result<(), (String, Position)> {
    let mut bracket = vec![];
    let mut need_number = true;
    for (index, &token) in stream.into_iter().enumerate() {
        match token {
            Reserved(_) => {
                if need_number {
                    return Err((
                        "fail to parse: need number here.".to_string(),
                        Position(index),
                    ));
                }
                need_number = true;
            }
            Number(_) => {
                if !need_number {
                    return Err((
                        "fail to parse: need operator here.".to_string(),
                        Position(index),
                    ));
                }
                need_number = false;
            }
            LeftBra => {
                bracket.push(index);
            }
            RightBra => {
                if bracket.pop().is_none() {
                    return Err((
                        "fail to parse: this bracker doesn't match.".to_string(),
                        Position(index),
                    ));
                }
            }
            _ => {
                if need_number {
                    return Err((
                        "fail to parse: need number here.".to_string(),
                        Position(index),
                    ));
                }
                break;
            }
        }
    }
    if let Some(index) = bracket.pop() {
        Err((
            "fail to parse: this bracket doesn't match.".to_string(),
            Position(index),
        ))
    } else {
        Ok(())
    }
}

pub fn add_sub_space(stream: &TokenStream) -> Result<String, (String, Position)> {
    // verify token sequence
    verify_stream(stream)?;

    let mut ret = String::new();
    ret.push_str(".intel_syntax noprefix\n");
    if cfg!(target_os = "linux") {
        ret.push_str(".global main\n\n");
        ret.push_str("main:\n");
    } else {
        ret.push_str(".global _main\n\n");
        ret.push_str("_main:\n");
    }
    ret.push_str("  mov rax, ");

    for &token in stream {
        match token {
            Reserved(operator) if operator == Operator::Add => ret.push_str("  add rax, "),
            Reserved(_) => ret.push_str("  sub rax, "),
            Number(number) => ret.push_str(&format!("{}\n", number)),
            Eof => {
                break;
            }
            _ => unreachable!(),
        }
    }

    ret.push_str("  ret\n\n");
    Ok(ret)
}

pub fn add_sub(program: &str) -> String {
    let mut ret = String::new();
    ret.push_str(".intel_syntax noprefix\n");
    if cfg!(target_os = "linux") {
        ret.push_str(".global main\n\n");
        ret.push_str("main:\n");
    } else {
        ret.push_str(".global _main\n\n");
        ret.push_str("_main:\n");
    }
    ret.push_str("  mov rax, ");
    let mut number = 0;
    for c in program.chars() {
        if c == '+' {
            ret.push_str(&format!("{}\n  add rax, ", number));
            number = 0;
        } else if c == '-' {
            ret.push_str(&format!("{}\n  sub rax, ", number));
            number = 0;
        } else if c.is_numeric() {
            number *= 10;
            number += (c as u8 - b'0') as usize;
        } else {
            println!("unexpected input: {}.", c);
            process::exit(1);
        }
    }
    ret.push_str(&format!("{}\n", number));
    ret.push_str("  ret\n\n");
    ret
}

pub fn return_number(number: &str) -> String {
    let number = number.parse::<i64>().expect("fail to convert to number");
    let mut ret = String::new();
    ret.push_str(".intel_syntax noprefix\n");
    if cfg!(target_os = "linux") {
        ret.push_str(".global main\n\n");
        ret.push_str("main:\n");
    } else {
        ret.push_str(".global _main\n\n");
        ret.push_str("_main:\n");
    }
    ret.push_str(&format!("    mov rax, {}\n", number));
    ret.push_str("    ret\n\n");
    ret
}

#[cfg(test)]
mod tests_parser {
    use super::*;
    use process::Command;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn for_expr01() {
        let cases = vec![
            "5+20-4",
            "23 - 8+5- 3",
            "1 + 2 * 3",
            "0",
            "(4 + 3) / 7 + 1 * (4 - 2)",
        ];
        let answers = vec![
            Sub(
                Box::new(Add(Box::new(Num(5)), Box::new(Num(20)))),
                Box::new(Num(4)),
            ),
            Sub(
                Box::new(Add(
                    Box::new(Sub(Box::new(Num(23)), Box::new(Num(8)))),
                    Box::new(Num(5)),
                )),
                Box::new(Num(3)),
            ),
            Add(
                Box::new(Num(1)),
                Box::new(Mul(Box::new(Num(2)), Box::new(Num(3)))),
            ),
            Num(0),
            Add(
                Box::new(Div(
                    Box::new(Add(Box::new(Num(4)), Box::new(Num(3)))),
                    Box::new(Num(7)),
                )),
                Box::new(Mul(
                    Box::new(Num(1)),
                    Box::new(Sub(Box::new(Num(4)), Box::new(Num(2)))),
                )),
            ),
        ];
        for (case, answer) in cases
            .into_iter()
            .map(|s| s.to_string())
            .zip(answers.into_iter())
        {
            let mut stream = TokenStream::tokenize01(case).unwrap();
            let ast = expr01(&mut stream);
            assert_eq!(ast, answer);
        }
    }

    #[test]
    fn for_add_sub_space() {
        let cases = vec!["5+20-4", "23 - 8+5- 3"];
        let answers = vec![21, 17];
        for (case, answer) in cases
            .into_iter()
            .map(|s| s.to_string())
            .zip(answers.into_iter())
        {
            let stream = TokenStream::tokenize01(case).unwrap();
            let program = add_sub_space(&stream).unwrap();
            let mut file = File::create("test03.s").unwrap();
            write!(file, "{}", program).unwrap();
            file.flush().unwrap();
            let out = Command::new("sh")
                .arg("-c")
                .arg(&format!("cc -o test03 test03.s; ./test03; echo $?",))
                .output()
                .unwrap()
                .stdout;
            let statement = std::str::from_utf8(&out).unwrap();
            assert_eq!(statement.trim().parse::<i64>().unwrap(), answer);
            Command::new("sh")
                .arg("-c")
                .arg("rm test03.s; rm test03")
                .output()
                .unwrap();
        }
    }

    #[test]
    fn for_add_sub_space_panic_number() {
        let cases = vec!["23 - 8+5-+ 3"];
        let answers = vec![17];
        for (case, _answer) in cases
            .into_iter()
            .map(|s| s.to_string())
            .zip(answers.into_iter())
        {
            let stream = TokenStream::tokenize01(case).unwrap();
            assert_eq!(
                add_sub_space(&stream),
                Err(("fail to parse: need number here.".to_string(), Position(6)))
            );
        }
    }

    #[test]
    fn for_add_sub_space_panic_operator() {
        let cases = vec!["23 - 8+5 3"];
        let answers = vec![17];
        for (case, _answer) in cases
            .into_iter()
            .map(|s| s.to_string())
            .zip(answers.into_iter())
        {
            let stream = TokenStream::tokenize01(case).unwrap();
            assert_eq!(
                add_sub_space(&stream),
                Err((
                    "fail to parse: need operator here.".to_string(),
                    Position(5)
                ))
            );
        }
    }

    #[test]
    fn for_add_sub() {
        let cases = vec!["5+20-4"];
        let answers = vec![21];
        for (case, answer) in cases.into_iter().zip(answers.into_iter()) {
            let program = add_sub(case);
            let mut file = File::create("test02.s").unwrap();
            write!(file, "{}", program).unwrap();
            file.flush().unwrap();
            let out = Command::new("sh")
                .arg("-c")
                .arg(&format!("cc -o test02 test02.s; ./test02; echo $?",))
                .output()
                .unwrap()
                .stdout;
            let statement = std::str::from_utf8(&out).unwrap();
            assert_eq!(statement.trim().parse::<i64>().unwrap(), answer);
            Command::new("sh")
                .arg("-c")
                .arg("rm test02.s; rm test02")
                .output()
                .unwrap();
        }
    }

    #[test]
    fn for_return_number() {
        let cases = vec!["0", "42", "255"];
        let answers = vec![0, 42, 255];
        for (case, answer) in cases.into_iter().zip(answers.into_iter()) {
            let program = return_number(case);
            let out = Command::new("sh")
                .arg("-c")
                .arg(&format!(
                    "echo \"{}\" > test01.s; cc -o test01 test01.s; ./test01; echo $?",
                    program
                ))
                .output()
                .unwrap()
                .stdout;
            let statement = std::str::from_utf8(&out).unwrap();
            assert_eq!(statement.trim().parse::<i64>().unwrap(), answer);
            Command::new("sh")
                .arg("-c")
                .arg("rm test01.s; rm test01")
                .output()
                .unwrap();
        }
    }
}
