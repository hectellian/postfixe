use crate::utils::lexer::*;

fn calc(v1: i32, op: char, v2: i32) -> i32 {
    match op {
            '^' => {
                if v2 >= 0 {
                    v1.pow(v2.unsigned_abs())
                } else {
                    1/v1.pow(v2.unsigned_abs())
                }
            },
            '*' => v1*v2,
            '/' => v1/v2,
            '+' => v1+v2,
            '-' => v1-v2,
            _ => 0,
    }
}

pub fn eval(postfixe: Vec<Token>) -> Result<i32, &'static str> {
    let mut stack = Vec::new();

    for element in postfixe {
        match element {
            TokenKind::Integer(v) => stack.push(v.parse().unwrap_or(0)),
            TokenKind::Real(v) => stack.push(v.parse().unwrap_or(0)),
            TokenKind::Operator{ raw , .. } => {
                let op1 = stack.pop();
                let op2 = stack.pop();
                let res = calc(op2.unwrap(), raw, op1.unwrap());
                stack.push(res);

                println!("calc: {:?} {:?} {:?} = {:?}", op2.unwrap(), raw, op1.unwrap(), res);
            },
            TokenKind::EOF => break,
            _ => return Err("Invalid Format: Numeric Postfixe Format Expected")
        }
    }

    let evaluated: i32 = stack.pop().unwrap();

    Ok(evaluated)
}