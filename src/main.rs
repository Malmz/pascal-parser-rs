#![feature(slice_patterns, advanced_slice_patterns)]

mod parser;

use parser::parser::evaluate;
use parser::ast::visitors::expression::Interpreter;

fn main() {
	use std::io;
	loop {
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Error reading input");
		input.pop();
		let result = evaluate(input).unwrap();
		println!("{}", result);
		let mut interpreter = Interpreter::new();
		result.accept(&mut interpreter);
		println!("{}", interpreter.result().unwrap());
	}
	
}