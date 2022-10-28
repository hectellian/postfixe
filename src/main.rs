mod utils;

use crate::utils::lexer::*;
use crate::utils::postfixe::*;

fn main() {
    let expression = String::from("A + (B / C - D ^ E )");

    let l = Lexer::from(expression);
    let t = l.tokenize().unwrap();
    println!("algebric expression: {:?}", t);

    print!("\n");

    let p = postfixe(t);
    println!("postfixe expression: {:?}", p);
}