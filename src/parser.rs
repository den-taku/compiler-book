use crate::lexer::Operator::*;
use crate::lexer::Token::*;
use std::process;

pub fn add_sub_space(stream: &crate::lexer::TokenStream) -> String {
    // verify token sequence
    let mut need_number = true;
    for &token in stream {
        match token {
            Reserved(_) => {
                if need_number {
                    panic!("fail to parse: need number here.")
                }
                need_number = true;
            }
            Number(_) => {
                if !need_number {
                    panic!("fail to parse: need operator here.")
                }
                need_number = false;
            }
            _ => {
                if need_number {
                    panic!("fail to parse: need number here.")
                }
                break;
            }
        }
    }

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
            Reserved(operator) if operator == Add => ret.push_str("  add rax, "),
            Reserved(_) => ret.push_str("  sub rax, "),
            Number(number) => ret.push_str(&format!("{}\n", number)),
            Eof => {
                break;
            }
        }
    }

    ret.push_str("  ret\n\n");
    ret
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
    use crate::lexer::*;
    use process::Command;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn for_add_sub_space() {
        let cases = vec!["5+20-4", "23 - 8+5- 3"];
        let answers = vec![21, 17];
        for (case, answer) in cases
            .into_iter()
            .map(|s| s.to_string())
            .zip(answers.into_iter())
        {
            let stream = TokenStream::tokenize01(case);
            let program = add_sub_space(&stream);
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
    #[should_panic(expected = "fail to parse: need number here.")]
    fn for_add_sub_space_panic_number() {
        let cases = vec!["23 - 8+5-+ 3"];
        let answers = vec![17];
        for (case, _answer) in cases
            .into_iter()
            .map(|s| s.to_string())
            .zip(answers.into_iter())
        {
            let stream = TokenStream::tokenize01(case);
            let _program = add_sub_space(&stream);
        }
    }

    #[test]
    #[should_panic(expected = "fail to parse: need operator here.")]
    fn for_add_sub_space_panic_operator() {
        let cases = vec!["23 - 8+5 3"];
        let answers = vec![17];
        for (case, _answer) in cases
            .into_iter()
            .map(|s| s.to_string())
            .zip(answers.into_iter())
        {
            let stream = TokenStream::tokenize01(case);
            let _program = add_sub_space(&stream);
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
