use super::*;

#[test]
fn only_numerical_values() {
    let expression = String::from("2 * (4^2 + 42/3)");
    let l = Lexer::from(expression);
    let t = l.tokenize().unwrap();
    let p = postfixe(t);
    let e = eval(p);
    match e {
        Ok(s) => assert_eq!(s, 60.0),
        Err(err) => println!("{:?}", err)
    }
}

#[test]
fn empty_expression() {
    let expression = String::from(" ");
    let l = Lexer::from(expression);
    let t = l.tokenize().unwrap();
    let p = postfixe(t);
    let e = eval(p);
    match e {
        Ok(s) => assert_eq!(s, 0.0),
        Err(err) => println!("{:?}", err)
    }
}