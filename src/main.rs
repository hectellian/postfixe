mod utils;

use crate::utils::lexer::*;
use crate::utils::postfixe::*;
use crate::utils::eval::*;

fn main() {
    let expression = String::from("2 * (4^2 + 42/3)");

    let l = Lexer::from(expression);
    let t = l.tokenize().unwrap();
    println!("algebric expression: {:?}", t);

    print!("\n");

    let p = postfixe(t);
    println!("postfixe expression: {:?}", p);

    print!("\n");

    let e = eval(p);
    println!("evalued expression result: {:?}", e);
}