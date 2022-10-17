mod utils;

use crate::utils::stack::*;
use crate::utils::lexer::*;

fn main() {
    let mut stack: Stack<&str> = Stack::new();

    stack.push("(");
    stack.push("(");
    stack.push("*");

    println!("Inside the Stack: {:#?}", stack);
    println!("Last element is: {:#?}", stack.peek());

    let item = stack.pop();
    
    println!("Inside the Stack after the pop: {:#?}", stack);
    println!("What about the pop (item): {:?}", item);

    let l = Lexer::from(String::from("Trying a = 2+b"));
    println!("{:?}",l.tokenize());
}