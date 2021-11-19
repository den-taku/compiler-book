use Operator::*;
use Token::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TokenStream {
    sequence: std::collections::LinkedList<Token>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Token {
    Reserved(Operator),
    Number(i64),
    Eof,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operator {
    Add,
    Sub,
}

impl TokenStream {
    pub fn tokenize01(mut program: String) -> Self {
        let mut sequence = std::collections::LinkedList::<Token>::new();
        while !program.is_empty() {
            // lex as number
            let (string, ret) = TokenStream::consume_number(program);
            program = string;
            if let Some(token) = ret {
                sequence.push_back(token);
                continue;
            }

            // lex as operator
            let (string, ret) = TokenStream::consume_operator(program);
            program = string;
            if let Some(token) = ret {
                sequence.push_back(token);
                continue;
            }

            // consume whitespaces
            let (string, ret) = TokenStream::consume_whitespace(program);
            program = string;
            if ret.is_some() {
                continue;
            }

            // fail to lex
            panic!("fail to lex. left: {}.", program);
        }
        if sequence.is_empty() {
            panic!("fail to lex. need some charactors without whitespace.")
        }
        sequence.push_back(Eof);
        Self { sequence }
    }

    fn consume_number(buffer: String) -> (String, Option<Token>) {
        let digit = buffer.chars().take_while(|c| c.is_ascii_digit()).count();
        let mut chars = buffer.chars();
        let number = chars.by_ref().take(digit).collect::<String>();
        let buffer = chars.collect::<String>();
        (
            buffer,
            if number.is_empty() {
                None
            } else {
                Some(Number(
                    number.parse::<i64>().expect("fail to parse number."),
                ))
            },
        )
    }

    fn consume_operator(buffer: String) -> (String, Option<Token>) {
        let mut chars = buffer.chars();
        match chars.by_ref().peekable().peek() {
            Some(op) if op == &'+' => ({ chars.collect::<String>() }, Some(Reserved(Add))),
            Some(op) if op == &'-' => ({ chars.collect::<String>() }, Some(Reserved(Sub))),
            _ => (buffer, None),
        }
    }

    fn consume_whitespace(buffer: String) -> (String, Option<Token>) {
        let spaces = buffer.chars().take_while(|c| c.is_whitespace()).count();
        (
            buffer.chars().skip(spaces).collect::<String>(),
            if spaces == 0 { None } else { Some(Eof) },
        )
    }
}

#[cfg(test)]
mod test_lexer {
    use super::*;

    #[test]
    fn for_tokenize() {
        let cases = vec!["5+20-4", "23 - 8+ 5-   3 + 56 + 9 - 8", "0"];
        let answers = vec![
            vec![
                Number(5),
                Reserved(Add),
                Number(20),
                Reserved(Sub),
                Number(4),
                Eof,
            ],
            vec![
                Number(23),
                Reserved(Sub),
                Number(8),
                Reserved(Add),
                Number(5),
                Reserved(Sub),
                Number(3),
                Reserved(Add),
                Number(56),
                Reserved(Add),
                Number(9),
                Reserved(Sub),
                Number(8),
                Eof,
            ],
            vec![Number(0), Eof],
        ];
        for (case, answer) in cases
            .into_iter()
            .map(|s| s.to_string())
            .zip(answers.into_iter().map(|tokens| TokenStream {
                sequence: tokens.into_iter().collect(),
            }))
        {
            assert_eq!(TokenStream::tokenize01(case), answer);
        }
        assert_eq!(1 + 2, 3);
    }

    #[test]
    #[should_panic(expected = "fail to lex. need some charactors without whitespace.")]
    fn for_tokenize_panic_empty() {
        let program = " \n   ".to_string();
        let _ = TokenStream::tokenize01(program);
    }

    #[test]
    #[should_panic(expected = "fail to lex. left: a + 89.")]
    fn for_tokenize_panic_invalid() {
        let program = "12 + 2 - a + 89".to_string();
        let _ = TokenStream::tokenize01(program);
    }
}
