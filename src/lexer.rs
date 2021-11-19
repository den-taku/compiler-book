use crate::error::*;
use std::iter::IntoIterator;
use Operator::*;
use Token::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TokenStream {
    pub sequence: std::collections::LinkedList<Token>,
    pub position: std::collections::LinkedList<usize>,
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

impl<'a> IntoIterator for &'a TokenStream {
    type Item = &'a Token;
    type IntoIter = std::collections::linked_list::Iter<'a, Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.sequence.iter()
    }
}

impl TokenStream {
    pub fn tokenize01(mut program: String) -> Result<Self, (String, Byte)> {
        let mut sequence = std::collections::LinkedList::<Token>::new();
        let mut position = std::collections::LinkedList::<usize>::new();
        let mut start_at = 0usize;
        while !program.is_empty() {
            // lex as number
            let (string, ret, width) = TokenStream::consume_number(program);
            program = string;
            if let Some(token) = ret {
                sequence.push_back(token);
                position.push_back(start_at);
                start_at += width;
                continue;
            }

            // lex as operator
            let (string, ret, width) = TokenStream::consume_operator(program);
            program = string;
            if let Some(token) = ret {
                sequence.push_back(token);
                position.push_back(start_at);
                start_at += width;
                continue;
            }

            // consume whitespaces
            let (string, ret, width) = TokenStream::consume_whitespace(program);
            program = string;
            if ret.is_some() {
                start_at += width;
                continue;
            }

            // fail to lex
            return Err((format!("fail to lex. left: {}.", program), Byte(start_at)));
        }
        if sequence.is_empty() {
            return Err((
                "fail to lex. need some charactors without whitespace.".to_string(),
                Byte(start_at),
            ));
        }
        sequence.push_back(Eof);
        Ok(Self { sequence, position })
    }

    fn consume_number(buffer: String) -> (String, Option<Token>, usize) {
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
            digit,
        )
    }

    fn consume_operator(buffer: String) -> (String, Option<Token>, usize) {
        let mut chars = buffer.chars();
        match chars.by_ref().peekable().peek() {
            Some(op) if op == &'+' => ({ chars.collect::<String>() }, Some(Reserved(Add)), 1),
            Some(op) if op == &'-' => ({ chars.collect::<String>() }, Some(Reserved(Sub)), 1),
            _ => (buffer, None, 0),
        }
    }

    fn consume_whitespace(buffer: String) -> (String, Option<Token>, usize) {
        let spaces = buffer.chars().take_while(|c| c.is_whitespace()).count();
        (
            buffer.chars().skip(spaces).collect::<String>(),
            if spaces == 0 { None } else { Some(Eof) },
            spaces,
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
            (
                vec![
                    Number(5),
                    Reserved(Add),
                    Number(20),
                    Reserved(Sub),
                    Number(4),
                    Eof,
                ],
                vec![0, 1, 2, 4, 5],
            ),
            (
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
                vec![0, 3, 5, 6, 8, 9, 13, 15, 17, 20, 22, 24, 26],
            ),
            (vec![Number(0), Eof], vec![0]),
        ];
        for (case, answer) in cases
            .into_iter()
            .map(|s| s.to_string())
            .zip(answers.into_iter().map(|tokens| TokenStream {
                sequence: tokens.0.into_iter().collect(),
                position: tokens.1.into_iter().collect(),
            }))
        {
            assert_eq!(TokenStream::tokenize01(case), Ok(answer));
        }
        assert_eq!(1 + 2, 3);
    }

    #[test]
    fn for_tokenize_panic_empty() {
        let program = " \n   ".to_string();
        assert_eq!(
            TokenStream::tokenize01(program),
            Err((
                "fail to lex. need some charactors without whitespace.".to_string(),
                Byte(5)
            ))
        );
    }

    #[test]
    fn for_tokenize_panic_invalid() {
        let program = "12 + 2 - a + 89".to_string();
        assert_eq!(
            TokenStream::tokenize01(program),
            Err(("fail to lex. left: a + 89.".to_string(), Byte(9)))
        );
    }
}
