use crate::error::Position;
use crate::lexer::{Token::*, TokenStream, Word};

// TODO:
pub fn verify_stream(stream: &TokenStream) -> Result<(), (String, Position)> {
    let mut bracket = vec![];
    let mut need_number = true;
    let mut count_unary = 0;
    for (index, &token) in stream.into_iter().enumerate() {
        match token {
            Reserved(op) if op == Word::Add || op == Word::Sub => {
                need_number = true;
                if count_unary >= 1 {
                    return Err((
                        "fail to parse: use unary only once.".to_string(),
                        Position(index),
                    ));
                }
                count_unary += 1;
            }
            Reserved(Word::LeftBra) => {
                bracket.push(index);
                count_unary = 0;
            }
            Reserved(Word::RightBra) => {
                if bracket.pop().is_none() {
                    return Err((
                        "fail to parse: this bracker doesn't match.".to_string(),
                        Position(index),
                    ));
                }
                count_unary = 0;
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
            }
            Number(_) => {
                if !need_number {
                    return Err((
                        "fail to parse: need operator here.".to_string(),
                        Position(index),
                    ));
                }
                need_number = false;
                count_unary = 0;
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
}
