//pub mod lexer;
pub mod lexer;
pub mod lexer2;
//pub mod interpreter;
//pub mod interpreter2;
//pub mod interpreter3;
pub mod ast;
//pub mod parser;
pub mod parser;
//use std;

#[derive(Clone ,Copy, Debug)]
pub enum Token {
	Integer(i32),
	Plus,
	Minus,
	Mult,
	Div,
	LParens,
	RParens,
}

pub type Tokens<'a> = &'a[Token];

pub fn pop(tokens: Tokens) -> Tokens { &tokens[1..] }

//pub type Result<T> = std::result::Result<T, String>;
