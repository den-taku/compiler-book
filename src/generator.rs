use crate::parser::{Node, Node::*};

pub fn generate_program03(nodes: &[Node]) -> String {
    let mut buffer = String::new();

    buffer.push_str(".intel_syntax noprefix\n");
    if cfg!(target_os = "linux") {
        buffer.push_str(".global main\n\n");
        buffer.push_str("main:\n");
    } else {
        buffer.push_str(".global _main\n\n");
        buffer.push_str("_main:\n");
    }

    // reserve stack for local value
    buffer.push_str("   push rbp\n");
    buffer.push_str("   mov rbp, rsp\n");
    buffer.push_str("   sub rsp, 208\n");

    for node in nodes {
        generator(node, &mut buffer);
        buffer.push_str("   pop rax\n");
    }

    buffer.push_str("   mov rsp, rbp\n");
    buffer.push_str("   pop rbp\n");
    buffer.push_str("   ret\n");

    buffer
}

pub fn generator(node: &Node, buffer: &mut String) {
    unimplemented!()
}

pub fn generate_lvalue(node: &Node, buffer: &mut String) {
    match node {
        LVar(offset) => {
            buffer.push_str("   mov rax, rbp\n");
            buffer.push_str(&format!("   sub rax, {}\n", offset));
            buffer.push_str("   push rax\n");
        }
        _ => unreachable!(),
    }
}

pub fn generate_program02(node: &Node) -> String {
    let mut buffer = String::new();

    buffer.push_str(".intel_syntax noprefix\n");
    if cfg!(target_os = "linux") {
        buffer.push_str(".global main\n\n");
        buffer.push_str("main:\n");
    } else {
        buffer.push_str(".global _main\n\n");
        buffer.push_str("_main:\n");
    }

    generate_arithmetics_compare(node, &mut buffer);

    buffer.push_str("   pop rax\n");
    buffer.push_str("   ret\n");

    buffer
}

pub fn generate_arithmetics_compare(node: &Node, buffer: &mut String) {
    match node {
        Num(number) => {
            buffer.push_str(&format!("   push {}\n", number));
        }
        Add(left, right)
        | Sub(left, right)
        | Mul(left, right)
        | Div(left, right)
        | Eq(left, right)
        | Ne(left, right)
        | Le(left, right)
        | Lt(left, right) => {
            // first push left value
            generate_arithmetics_compare(left, buffer);
            // next push right value on the left value
            generate_arithmetics_compare(right, buffer);

            // right value -> rdi
            buffer.push_str("   pop rdi\n");
            // left value -> rax
            buffer.push_str("   pop rax\n");

            match node {
                Add(_, _) => buffer.push_str("   add rax, rdi\n"),
                Sub(_, _) => buffer.push_str("   sub rax, rdi\n"),
                Mul(_, _) => buffer.push_str("   imul rax, rdi\n"),
                Div(_, _) => {
                    buffer.push_str("   cqo\n");
                    buffer.push_str("   idiv rdi\n")
                }
                Eq(_, _) | Ne(_, _) | Le(_, _) | Lt(_, _) => {
                    buffer.push_str("   cmp rax, rdi\n");
                    match node {
                        Eq(_, _) => {
                            buffer.push_str("   sete al\n");
                        }
                        Ne(_, _) => {
                            buffer.push_str("   setne al\n");
                        }
                        Le(_, _) => {
                            buffer.push_str("   setle al\n");
                        }
                        Lt(_, _) => {
                            buffer.push_str("   setl al\n");
                        }
                        _ => unreachable!(),
                    }
                    if cfg!(target_os = "linux") {
                        buffer.push_str("   movzb rax, al\n");
                    } else {
                        buffer.push_str("   movzx rax, al\n");
                    }
                }

                Num(_) => unreachable!(),
                _ => unreachable!(),
            }

            buffer.push_str("   push rax\n")
        }
        _ => unreachable!(),
    }
}

pub fn generate_program01(node: &Node) -> String {
    let mut buffer = String::new();

    buffer.push_str(".intel_syntax noprefix\n");
    if cfg!(target_os = "linux") {
        buffer.push_str(".global main\n\n");
        buffer.push_str("main:\n");
    } else {
        buffer.push_str(".global _main\n\n");
        buffer.push_str("_main:\n");
    }

    generate_arithmetics(node, &mut buffer);

    buffer.push_str("   pop rax\n");
    buffer.push_str("   ret\n");

    buffer
}

pub fn generate_arithmetics(node: &Node, buffer: &mut String) {
    match node {
        Num(number) => {
            buffer.push_str(&format!("   push {}\n", number));
        }
        Add(left, right) | Sub(left, right) | Mul(left, right) | Div(left, right) => {
            // first push left value
            generate_arithmetics(left, buffer);
            // next push right value on the left value
            generate_arithmetics(right, buffer);

            // right value -> rdi
            buffer.push_str("   pop rdi\n");
            // left value -> rax
            buffer.push_str("   pop rax\n");

            match node {
                Add(_, _) => buffer.push_str("   add rax, rdi\n"),
                Sub(_, _) => buffer.push_str("   sub rax, rdi\n"),
                Mul(_, _) => buffer.push_str("   imul rax, rdi\n"),
                Div(_, _) => {
                    buffer.push_str("   cqo\n");
                    buffer.push_str("   idiv rdi\n")
                }
                Num(_) => unreachable!(),
                _ => unreachable!(),
            }

            buffer.push_str("   push rax\n")
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests_generator {
    use super::*;
    use crate::lexer::TokenStream;
    use crate::parser::*;
    use std::fs::File;
    use std::io::Write;
    use std::process::Command;

    #[test]
    fn for_generate_program02() {
        let cases = vec![
            "5+20-4",
            "23 - 8+5- 3",
            "1 + 2 * 3",
            "0",
            "(4 + 3) / 7 + 1 * (4 - 2)",
            "((4    +3) /  7 +4) *(4 -2 +   3 )",
            "-3*+5+20",
            "0==1",
            "35==35",
            "0!=1",
            "0 != 0",
            "0 < 1",
            "1 < 1",
            "5 <= 123",
            "5 <= 5",
            "5 > 5",
            "5 >= 5",
            "127 >= 0",
            "0 >= 1",
            "0 == 0 == 0",
            " 7 > 0 > 0",
            "0 < 0 < 7",
            "((((4 + 3) / 7 + 4) * (4 - 2) == 10) > 0) * 120",
            "(((4 + 3) / 7 + 4) * (4 - 2) == 10 > 0) * 120",
        ];
        let answers = vec![
            21, 17, 7, 0, 3, 25, 5, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 120, 0,
        ];
        for (case, answer) in cases
            .into_iter()
            .map(|s| s.to_string())
            .zip(answers.into_iter())
        {
            let mut stream = TokenStream::tokenize01(case).unwrap();
            let ast = expr(&mut stream);
            let program = generate_program02(&ast);
            let mut file = File::create("test05.s").unwrap();
            write!(file, "{}", program).unwrap();
            file.flush().unwrap();
            let out = Command::new("sh")
                .arg("-c")
                .arg(&format!("cc -o test05 test05.s; ./test05; echo $?",))
                .output()
                .unwrap()
                .stdout;
            let statement = std::str::from_utf8(&out).unwrap();
            assert_eq!(statement.trim().parse::<i64>().unwrap(), answer);
            Command::new("sh")
                .arg("-c")
                .arg("rm test05.s; rm test05")
                .output()
                .unwrap();
        }
    }

    #[test]
    fn for_generate_program01() {
        let cases = vec![
            "5+20-4",
            "23 - 8+5- 3",
            "1 + 2 * 3",
            "0",
            "(4 + 3) / 7 + 1 * (4 - 2)",
            "((4    +3) /  7 +4) *(4 -2 +   3 )",
            "-3*+5+20",
        ];
        let answers = vec![21, 17, 7, 0, 3, 25, 5];
        for (case, answer) in cases
            .into_iter()
            .map(|s| s.to_string())
            .zip(answers.into_iter())
        {
            let mut stream = TokenStream::tokenize01(case).unwrap();
            let ast = expr(&mut stream);
            let program = generate_program01(&ast);
            let mut file = File::create("test04.s").unwrap();
            write!(file, "{}", program).unwrap();
            file.flush().unwrap();
            let out = Command::new("sh")
                .arg("-c")
                .arg(&format!("cc -o test04 test04.s; ./test04; echo $?",))
                .output()
                .unwrap()
                .stdout;
            let statement = std::str::from_utf8(&out).unwrap();
            assert_eq!(statement.trim().parse::<i64>().unwrap(), answer);
            Command::new("sh")
                .arg("-c")
                .arg("rm test04.s; rm test04")
                .output()
                .unwrap();
        }
    }

    #[test]
    fn for_generate_arithmetics() {
        let cases = vec![
            "5+20-4",
            "23 - 8+5- 3",
            "1 + 2 * 3",
            "0",
            "(4 + 3) / 7 + 1 * (4 - 2)",
        ];
        let answers = vec![
            "   push 5\
           \n   push 20\
           \n   pop rdi\
           \n   pop rax\
           \n   add rax, rdi\
           \n   push rax\
           \n   push 4\
           \n   pop rdi\
           \n   pop rax\
           \n   sub rax, rdi\
           \n   push rax\n",
            "   push 23\
           \n   push 8\
           \n   pop rdi\
           \n   pop rax\
           \n   sub rax, rdi\
           \n   push rax\
           \n   push 5\
           \n   pop rdi\
           \n   pop rax\
           \n   add rax, rdi\
           \n   push rax\
           \n   push 3\
           \n   pop rdi\
           \n   pop rax\
           \n   sub rax, rdi\
           \n   push rax\n",
            "   push 1\
           \n   push 2\
           \n   push 3\
           \n   pop rdi\
           \n   pop rax\
           \n   imul rax, rdi\
           \n   push rax\
           \n   pop rdi\
           \n   pop rax\
           \n   add rax, rdi\
           \n   push rax\n",
            "   push 0\n",
            "   push 4\
           \n   push 3\
           \n   pop rdi\
           \n   pop rax\
           \n   add rax, rdi\
           \n   push rax\
           \n   push 7\
           \n   pop rdi\
           \n   pop rax\
           \n   cqo\
           \n   idiv rdi\
           \n   push rax\
           \n   push 1\
           \n   push 4\
           \n   push 2\
           \n   pop rdi\
           \n   pop rax\
           \n   sub rax, rdi\
           \n   push rax\
           \n   pop rdi\
           \n   pop rax\
           \n   imul rax, rdi\
           \n   push rax\
           \n   pop rdi\
           \n   pop rax\
           \n   add rax, rdi\
           \n   push rax\n",
        ];
        for (case, answer) in cases
            .into_iter()
            .map(|s| s.to_string())
            .zip(answers.into_iter().map(|s| s.to_string()))
        {
            let mut stream = TokenStream::tokenize01(case).unwrap();
            let ast = expr(&mut stream);
            let mut buffer = String::new();
            generate_arithmetics(&ast, &mut buffer);
            assert_eq!(buffer, answer);
        }
    }
}
