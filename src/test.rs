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
fn negative_numbers(){
    let expression = String::from("-2 * (4^2 + 42/3)");
    let l = Lexer::from(expression);
    let t = l.tokenize().unwrap();
    let p = postfixe(t);
    let e = eval(p);
    match e {
        Ok(s) => assert_eq!(s, -60.0),
        Err(err) => println!("{:?}", err)
    }
}

#[test]
fn float_numbers(){
    let expression = String::from("( 256.0 + 95.3 ) * 10^5");
    let l = Lexer::from(expression);
    let t = l.tokenize().unwrap();
    let p = postfixe(t);
    let e = eval(p);
    match e {
        Ok(s) => assert_eq!(s, 35130000.0),
        Err(err) => println!("{:?}", err)
    }
}

#[test]
fn empty_expression() {
    let expression = String::from(" ");
    let l = Lexer::from(expression);
    let t = l.tokenize().ok().unwrap();
    let p = postfixe(t);
    let e = eval(p);
    match e {
        Ok(s) => assert_eq!(s, 0.0),
        Err(err) => println!("{:?}", err)
    }
}

#[test]
fn non_numerical_value() {
    let expression = String::from("a + 6");
    let l = Lexer::from(expression);
    let t = l.tokenize().ok().unwrap();
    let p = postfixe(t);
    let e = eval(p);
    match e {
        Ok(p) => println!("{}", p),
        Err(err) => assert_eq!(err, "Invalid Format: Numeric Postfixe Format Expected")
    }
}