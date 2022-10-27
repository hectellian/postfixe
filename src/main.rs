mod utils;

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

fn postfixe(tokens: Vec<Token>) -> Vec<Token> {
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
                    match p.unwrap() {
                        &TokenKind::OpenParenthesis => {stack.pop();break},
                        _ => {
                            let last = stack.pop();
                            postfixe.push(last.unwrap());
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
                        None => break
                    }
                }
            },
            _ => postfixe.push(element)
        }
    }

    // println!("{:#?}", stack);

    postfixe
}

fn main() {
    let expression = String::from("A + (B / C - D ^ E )");

    /* let mut stack: Stack<&str> = Stack::new();

    stack.push("(");
    stack.push("(");
    stack.push("*");

    println!("Inside the Stack: {:#?}", stack);
    println!("Last element is: {:#?}", stack.peek());

    let item = stack.pop();
    
    println!("Inside the Stack after the pop: {:#?}", stack);
    println!("What about the pop (item): {:?}", item); */

    let l = Lexer::from(expression);
    let t = l.tokenize().unwrap();
    println!("algebric expression: {:?}", t);
    print!("\n");
    let p = postfixe(t);
    println!("postfixe expression: {:?}", p);

    /* let op = &t[1];

    let opcost = cost(op);
    println!("Cost of {:?}: {:?}", op, opcost); */
}