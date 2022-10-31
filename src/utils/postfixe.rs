use crate::utils::lexer::*;

fn cost(op: &TokenKind) -> usize {
    match op {
            TokenKind::Operator { raw:'^', .. } => 3,
            TokenKind::Operator { raw:'*', .. } => 2,
            TokenKind::Operator { raw:'/', .. } => 2,
            TokenKind::Operator { raw:'+', .. } => 1,
            TokenKind::Operator { raw:'-', .. } => 1,
            _ => 0,
    }
}

pub fn postfixe(tokens: Vec<Token>) -> Vec<Token> {
    let mut stack: Vec<Token> = Vec::new();
    let mut postfixe: Vec<Token> = Vec::new();

    for element in tokens {
        match element {
            TokenKind::OpenParenthesis => stack.push(element),
            TokenKind::Operator{raw:_, ..} => {
                loop {
                    let p = stack.last();
                    let c = &element;
                    match p {
                        Some(p) => {
                            if cost(c) >= cost(p) {
                                stack.push(element);
                                break;
                            } else {
                                let last = stack.pop();
                                postfixe.push(last.unwrap());
                            }
                        },
                        None => {stack.push(element);break;},
                    }
                }
            },
            TokenKind::CloseParenthesis => {
                loop {
                    let p = stack.last();
                    match p.unwrap_or(&TokenKind::Integer(String::from("0"))) {
                        &TokenKind::OpenParenthesis => {stack.pop();break},
                        _ => {
                            let last = stack.pop();
                            postfixe.push(last.unwrap_or(TokenKind::Integer(String::from("0"))));
                        }  
                    }
                }
            },
            TokenKind::EOF => {
                loop {
                    let p = stack.last();
                    match p {
                        Some(_p) => {
                            let last = stack.pop();
                            postfixe.push(last.unwrap());
                        },
                        None => {postfixe.push(TokenKind::EOF);break}
                    }
                }
            },
            _ => postfixe.push(element)
        }
    }
    // println!("{:#?}", stack);
    postfixe
}
