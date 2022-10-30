use std::env;

mod utils;

use crate::utils::lexer::*;
use crate::utils::postfixe::*;
use crate::utils::eval::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let expression = String::from( &args[1..].concat() );
    
    let l = Lexer::from(expression);
    let t = l.tokenize().ok().unwrap();
    println!("algebric expression: {:?}", t);

    print!("\n");

    let p = postfixe(t);
    println!("postfixe expression: {:?}", p);

    print!("\n");

    let e = eval(p);
    println!("evalued expression result: {:?}", e);
}

#[cfg(test)]
mod test;