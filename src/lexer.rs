use crate::error::*;
use std::iter::IntoIterator;
use Token::*;
use Word::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TokenStream {
    pub sequence: std::collections::LinkedList<Token>,
    pub position: std::collections::LinkedList<usize>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Token {
    Reserved(Word),
    Ident(u64),
    Number(i64),
    SemiColon,
    Eof,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Word {
    LeftBra,
    RightBra,
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Le,
    Lt,
    Ge,
    Gt,
    Assign,
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
            // lex semicolon
            let (string, ret, width) = TokenStream::consume_semicolon(program);
            program = string;
            if let Some(token) = ret {
                sequence.push_back(token);
                position.push_back(start_at);
                start_at += width;
                continue;
            }

            // lex brackets
            let (string, ret, width) = TokenStream::consume_bracket(program);
            program = string;
            if let Some(token) = ret {
                sequence.push_back(token);
                position.push_back(start_at);
                start_at += width;
                continue;
            }

            // lex as number
            let (string, ret, width) = TokenStream::consume_number(program);
            program = string;
            if let Some(token) = ret {
                sequence.push_back(token);
                position.push_back(start_at);
                start_at += width;
                continue;
            }

            // lex as orderd operator
            let (string, ret, width) = TokenStream::consume_order(program);
            program = string;
            if let Some(token) = ret {
                sequence.push_back(token);
                position.push_back(start_at);
                start_at += width;
                continue;
            }

            // lex as alphabet
            let (string, ret, width) = TokenStream::consume_alphabetic(program);
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

    fn consume_bracket(buffer: String) -> (String, Option<Token>, usize) {
        let mut chars = buffer.chars();
        match chars.by_ref().peekable().peek() {
            Some(op) if op == &'(' => ({ chars.collect::<String>() }, Some(Reserved(LeftBra)), 1),
            Some(op) if op == &')' => ({ chars.collect::<String>() }, Some(Reserved(RightBra)), 1),
            _ => (buffer, None, 0),
        }
    }

    fn consume_semicolon(buffer: String) -> (String, Option<Token>, usize) {
        if let Some(c) = buffer.chars().next() {
            if c == ';' {
                (
                    buffer.chars().skip(1).collect::<String>(),
                    Some(SemiColon),
                    1,
                )
            } else {
                (buffer, None, 0)
            }
        } else {
            (buffer, None, 0)
        }
    }

    fn consume_alphabetic(buffer: String) -> (String, Option<Token>, usize) {
        let alphabets = buffer.chars().take_while(|c| c.is_alphabetic()).count();
        (
            buffer.chars().skip(alphabets).collect::<String>(),
            if alphabets == 0 {
                None
            } else {
                Some(Ident((buffer.chars().next().unwrap() as u8 - b'a') as u64))
            },
            alphabets,
        )
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

    fn consume_order(buffer: String) -> (String, Option<Token>, usize) {
        let mut chars = buffer.chars();
        match chars.by_ref().peekable().peek() {
            Some(or) if or == &'<' => match chars.by_ref().peekable().peek() {
                Some(eq) if eq == &'=' => (chars.collect::<String>(), Some(Reserved(Le)), 2),
                _ => (
                    buffer.chars().skip(1).collect::<String>(),
                    Some(Reserved(Lt)),
                    1,
                ),
            },
            Some(or) if or == &'>' => match chars.by_ref().peekable().peek() {
                Some(eq) if eq == &'=' => (chars.collect::<String>(), Some(Reserved(Ge)), 2),
                _ => (
                    buffer.chars().skip(1).collect::<String>(),
                    Some(Reserved(Gt)),
                    1,
                ),
            },
            Some(or) if or == &'=' => match chars.by_ref().peekable().peek() {
                Some(eq) if eq == &'=' => (chars.collect::<String>(), Some(Reserved(Eq)), 2),
                // fail to parse
                _ => (
                    buffer.chars().skip(1).collect::<String>(),
                    Some(Reserved(Eq)),
                    1,
                ),
            },
            Some(or) if or == &'!' => match chars.by_ref().peekable().peek() {
                Some(eq) if eq == &'=' => (chars.collect::<String>(), Some(Reserved(Ne)), 2),
                // fail to parse
                _ => (buffer, None, 0),
            },
            _ => (buffer, None, 0),
        }
    }

    fn consume_operator(buffer: String) -> (String, Option<Token>, usize) {
        let mut chars = buffer.chars();
        match chars.by_ref().peekable().peek() {
            Some(op) if op == &'+' => ({ chars.collect::<String>() }, Some(Reserved(Add)), 1),
            Some(op) if op == &'-' => ({ chars.collect::<String>() }, Some(Reserved(Sub)), 1),
            Some(op) if op == &'*' => ({ chars.collect::<String>() }, Some(Reserved(Mul)), 1),
            Some(op) if op == &'/' => ({ chars.collect::<String>() }, Some(Reserved(Div)), 1),
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
mod tests_lexer {
    use super::*;

    #[test]
    fn for_tokenize() {
        let cases = vec![
            "5+20-4",
            "23 - 8+ 5-   3 + 56 + 9 - 8",
            "0",
            "4 * 5 < 3 - 2",
            "-3==5",
        ];
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
            (
                vec![
                    Number(4),
                    Reserved(Mul),
                    Number(5),
                    Reserved(Lt),
                    Number(3),
                    Reserved(Sub),
                    Number(2),
                    Eof,
                ],
                vec![0, 2, 4, 6, 8, 10, 12],
            ),
            (
                vec![Reserved(Sub), Number(3), Reserved(Eq), Number(5), Eof],
                vec![0, 1, 2, 4],
            ),
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
        let program = "12 + 2 - ☀︎ + 89".to_string();
        assert_eq!(
            TokenStream::tokenize01(program),
            Err(("fail to lex. left: ☀︎ + 89.".to_string(), Byte(9)))
        );
    }
}
