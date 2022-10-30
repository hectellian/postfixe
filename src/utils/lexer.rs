//! Lexical analyzer Module

#![allow(dead_code,unused_variables)]

pub type Token = TokenKind;

#[derive(Debug)]
pub enum TokenKind {
    /** End of token stream */
    EOF,

    /** Operators type such as + or * */
    Operator{raw: char, kind: OperatorKind},

    /** The equal operator, reserved for asignation */
    Equal,

    /** Integers */
    Integer(String),

    /** Variables identifiers */
    Identifier(String),

    /** Real numbers */
    Real(String),

    /** Char chains: String */
    String(String),

    /** ( delimiting the opening of a parenthesis group */
    OpenParenthesis,

    /** ) delimiting the end of a parenthesis group */
    CloseParenthesis,

    /** ´ delimeting the opening of a String */
    OpenApostrophe,

    /** ` delimiting the end of a String */
    CloseApostrophe,

    /** New line token, a natural separator between inputs */
    NewLine,

    /** Unknown token in place of error */
    UnknownToken(String)
}

#[derive(Debug,Clone, Copy)]
pub enum OperatorKind {
    /** The multiplication Operator */
    Multiplier,

    /** The addition Operator */
    Adder,

    /** The division operator */
    Diviser,

    /** The power operator*/
    Power,

    /** The substraction operator */
    Substracter
}


/** The lexer object used to tokenize a given input */
#[derive(Debug)]
pub struct Lexer {
    /** The entry text */
    pub input: String,

    /** The Human readable line */
    pub cur_line: usize,

    /** The Human readable column */
    pub cur_col: usize,

    /** The cursor position from the start of input */
    pub codepoint_offset: usize
}


#[derive(Debug)]
pub enum LexerError {
    /** Unexpected end of file */
    UnexpectedEndOfFile,

    /** Unexpected number format */
    UnexpectedNumberFormat,
}

impl Lexer {

    pub fn new() -> Lexer {
        Lexer {
            cur_col:0,
            cur_line:0,
            codepoint_offset:0,
            input:String::new()
        }
    }

    pub fn from(input: String) -> Lexer {
        Lexer {
            cur_line:0,
            cur_col:0,
            codepoint_offset:0,
            input:input
        }
    }

    pub fn tokenize(&self) -> Result<Vec<Token>,LexerError> {

        let mut tokens = Vec::new();

        let mut char_ite = self.input.chars();

        let mut r = char::REPLACEMENT_CHARACTER;

        loop {
            let c:char;
            if r==char::REPLACEMENT_CHARACTER {
                c = char_ite.next().unwrap_or_else(|| '\0');
            } else {
                c=r;
                r=char::REPLACEMENT_CHARACTER;
            }
            match c {
                '\0' => {tokens.push(TokenKind::EOF); break;},
                ' ' => {print!("s"); continue},
                '=' => tokens.push(TokenKind::Equal),
                '+' => tokens.push(TokenKind::Operator { raw: '+', kind: OperatorKind::Adder }),
                '*' => tokens.push(TokenKind::Operator { raw: '*', kind: OperatorKind::Multiplier }),
                '^' => tokens.push(TokenKind::Operator { raw: '^', kind: OperatorKind::Power }),
                '/' => {tokens.push(TokenKind::Operator { raw: '/', kind: OperatorKind::Diviser }); print!("hi");},
                '-' => tokens.push(TokenKind::Operator { raw: '-', kind: OperatorKind::Substracter }),
                '(' => tokens.push(TokenKind::OpenParenthesis),
                ')' => tokens.push(TokenKind::CloseParenthesis),
                '\n' => tokens.push(TokenKind::NewLine),
                '`' => {
                        tokens.push(TokenKind::OpenApostrophe);
                        let mut s= String::new();
                        loop {
                            let c = char_ite.next();
                            match c {
                                None => {return Err::<Vec<Token>,LexerError>(LexerError::UnexpectedEndOfFile);},
                                Some(c) => {
                                    if c == '´' {
                                        tokens.push(TokenKind::String(s));
                                        tokens.push(TokenKind::CloseApostrophe);
                                        break;
                                    } else {
                                        s.push(c);
                                    }
                                }
                            }
                        }
                    },
                '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                    let mut s = String::from(c);
                    loop {
                        let c = char_ite.next();
                        match c {
                            None => {break;},
                            Some(c) => {
                                if !c.is_numeric() && c!='.' { 
                                    r=c;
                                    break;
                                } else {
                                    s.push(c);
                                }
                            }
                        }
                    }
                    
                    if let Some(end) = tokens.pop() {
                        match end {
                            TokenKind::Operator { raw, kind } => {
                                match kind {
                                    OperatorKind::Substracter =>{
                                        let nend = tokens.pop().unwrap_or(TokenKind::UnknownToken(String::new()));
                                        match nend {
                                            TokenKind::Real(_) | TokenKind::Integer(_) => {tokens.push(nend); tokens.push(end);},
                                            TokenKind::UnknownToken(_) => s.insert(0, '-'),
                                            _ => {tokens.push(nend); s.insert(0, '-')}
                                        }
                                    },
                                    _ => {tokens.push(end);}
                                }
                            },
                            _ => tokens.push(end)
                        }
                    }
                    if s.contains('.') {
                        tokens.push(TokenKind::Real(s));
                    } else {
                        tokens.push(TokenKind::Integer(s));
                    }
                },
                _ => {

                    if c.is_alphabetic() {
                        let mut s = String::from(c);
                        loop {
                            let c =char_ite.next();
                            match c {
                                None => {break;},
                                Some(c) => {
                                    if c.is_alphanumeric() {
                                        s.push(c);
                                    } else {
                                        r=c;
                                        break;
                                    }
                                }
                            }
                        }
                        tokens.push(TokenKind::Identifier(s));
                    } else {
                        tokens.push(TokenKind::UnknownToken(c.to_string()))
                    }
                }
            }
        }

        Ok(tokens)
    }

}