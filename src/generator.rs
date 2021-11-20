use crate::parser::{Node, Node::*};

pub fn generate_program(node: &Node) -> String {
    let mut buffer = String::new();
    generate_program_inner(node, &mut buffer);
    buffer
}

fn generate_program_inner(node: &Node, buffer: &mut String) {
    match node {
        Num(number) => {
            buffer.push_str(&format!("   push {}\n", number));
        }
        Add(left, right) | Sub(left, right) | Mul(left, right) | Div(left, right) => {
            // first push left value
            generate_program_inner(left, buffer);
            // next push right value on the left value
            generate_program_inner(right, buffer);

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
            }

            buffer.push_str("   push rax\n")
        }
    }
}

#[cfg(test)]
mod tests_generator {
    use super::*;
    use crate::lexer::TokenStream;
    use crate::parser::*;

    #[test]
    fn for_generator() {
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
            let ast = expr01(&mut stream);
            assert_eq!(generate_program(&ast), answer);
        }
    }
}
