use crate::error::Position;
use crate::lexer::{Token::*, TokenStream, Word};

pub fn verify_stream(stream: &TokenStream) -> Result<(), (String, Position)> {
    let mut bracket = vec![];
    let mut need_number = true;
    let mut count_unary = 0;
    let mut need_semicolon = false;
    for (index, &token) in stream.into_iter().enumerate() {
        match token {
            SemiColon => {
                if need_semicolon && need_number {
                    return Err((
                        "fail to parse: need number here.".to_string(),
                        Position(index),
                    ));
                } else if let Some(index) = bracket.pop() {
                    return Err((
                        "fail to parse: this bracket doesn't match.".to_string(),
                        Position(index),
                    ));
                } else {
                    bracket = vec![];
                    need_number = true;
                    count_unary = 0;
                    need_semicolon = false;
                }
            }
            Reserved(assign) if assign == Word::Assign => {
                need_number = true;
                need_semicolon = true;
            }
            Reserved(op) if op == Word::Add || op == Word::Sub => {
                need_number = true;
                if count_unary >= 1 {
                    return Err((
                        "fail to parse: use unary only once.".to_string(),
                        Position(index),
                    ));
                }
                count_unary += 1;
                need_semicolon = true;
            }
            Reserved(Word::LeftBra) => {
                bracket.push(index);
                count_unary = 0;
                need_semicolon = true;
            }
            Reserved(Word::RightBra) => {
                if bracket.pop().is_none() {
                    return Err((
                        "fail to parse: this bracket doesn't match.".to_string(),
                        Position(index),
                    ));
                }
                count_unary = 0;
                need_semicolon = true;
            }
            Reserved(_) => {
                if need_number {
                    return Err((
                        "fail to parse: need number here.".to_string(),
                        Position(index),
                    ));
                }
                need_number = true;
                count_unary = 0;
                need_semicolon = true;
            }
            Number(_) | Ident(_) => {
                if !need_number {
                    return Err((
                        "fail to parse: need operator here.".to_string(),
                        Position(index),
                    ));
                }
                need_number = false;
                count_unary = 0;
                need_semicolon = true;
            }

            Eof => {
                if need_semicolon && need_number {
                    return Err((
                        "fail to parse: need number here.".to_string(),
                        Position(index),
                    ));
                } else if need_semicolon {
                    return Err((
                        "fail to parse: need semicolon here.".to_string(),
                        Position(index),
                    ));
                }
                break;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests_static_check {
    use super::*;

    #[test]
    fn for_lacking_operator() {
        let cases = vec!["23 - 8+5 3"];
        let answers = vec![17];
        for (case, _answer) in cases
            .into_iter()
            .map(|s| s.to_string())
            .zip(answers.into_iter())
        {
            let stream = TokenStream::tokenize(case).unwrap();
            assert_eq!(
                verify_stream(&stream),
                Err((
                    "fail to parse: need operator here.".to_string(),
                    Position(5)
                ))
            );
        }
    }

    #[test]
    fn for_lacking_semicolon() {
        let cases = vec!["23 - 8+5"];
        let answers = vec![17];
        for (case, _answer) in cases
            .into_iter()
            .map(|s| s.to_string())
            .zip(answers.into_iter())
        {
            let stream = TokenStream::tokenize(case).unwrap();
            assert_eq!(
                verify_stream(&stream),
                Err((
                    "fail to parse: need semicolon here.".to_string(),
                    Position(5)
                ))
            );
        }
    }

    #[test]
    fn for_invalid_bracket() {
        let cases = vec![
            "(23 - 8+5;",
            "((((34 * 5) - 5) == 0);",
            "(23 - 8+5));",
            "(9));",
            ");",
            ";3+4;);",
        ];
        let errors = vec![
            ("fail to parse: this bracket doesn't match.", 0),
            ("fail to parse: this bracket doesn't match.", 0),
            ("fail to parse: this bracket doesn't match.", 7),
            ("fail to parse: this bracket doesn't match.", 3),
            ("fail to parse: this bracket doesn't match.", 0),
            ("fail to parse: this bracket doesn't match.", 5),
        ];
        for (case, error) in cases.into_iter().map(|s| s.to_string()).zip(
            errors
                .into_iter()
                .map(|(message, position)| Err((message.to_string(), Position(position)))),
        ) {
            let stream = TokenStream::tokenize(case).unwrap();
            assert_eq!(verify_stream(&stream), error);
        }
    }

    #[test]
    fn for_too_much_unary() {
        let cases = vec!["++9;", "a=3+4;b=+5;c=-+3"];
        let errors = vec![
            ("fail to parse: use unary only once.", 1),
            ("fail to parse: use unary only once.", 14),
        ];
        for (case, error) in cases.into_iter().map(|s| s.to_string()).zip(
            errors
                .into_iter()
                .map(|(message, position)| Err((message.to_string(), Position(position)))),
        ) {
            let stream = TokenStream::tokenize(case).unwrap();
            assert_eq!(verify_stream(&stream), error);
        }
    }

    #[test]
    fn for_need_number() {
        let cases = vec!["3+;", "a=3+4;b=+5;c=-3;3 ==", "a=;"];
        let errors = vec![
            ("fail to parse: need number here.", 2),
            ("fail to parse: need number here.", 18),
            ("fail to parse: need number here.", 2),
        ];
        for (case, error) in cases.into_iter().map(|s| s.to_string()).zip(
            errors
                .into_iter()
                .map(|(message, position)| Err((message.to_string(), Position(position)))),
        ) {
            let stream = TokenStream::tokenize(case).unwrap();
            assert_eq!(verify_stream(&stream), error);
        }
    }
}
