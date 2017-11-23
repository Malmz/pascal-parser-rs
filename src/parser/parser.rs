use std;
use super::Token;
use super::ast::{BinOp, Node, BinOperator, Leaf, UnaryOperator, UnaryOp};
use super::lexer::lexer;
type Result<I, O> = std::result::Result<(I, O), ParseError>;
type Tokens<'a> = &'a [Token];

#[derive(Debug)]
pub enum ParseError {
    None,
    Incomplete(String),
}

pub fn evaluate(text: String) -> std::result::Result<Box<Node>, ParseError> {
    let tokens = match lexer(text) {
        Some(val) => val,
        None => return Err(ParseError::Incomplete("Lexer error".to_owned()))
    };
    expr(tokens.as_slice()).map(|(val, _)| val)
}

fn expr(tokens: Tokens) -> Result<Box<Node>, Tokens> {
    let (mut lhs, mut outer_tokens) = term(tokens)?;
    loop {
        let (op, tokens) = match add(outer_tokens).or(sub(outer_tokens)) {
            Ok(val) => val,
            Err(_) => break
        };
        let (rhs, tokens) = term(tokens)?;
        outer_tokens = tokens;
        lhs = BinOp::new(op.to_bin(), lhs, rhs);
    }
    Ok((lhs, outer_tokens))
}

fn term(tokens: Tokens) -> Result<Box<Node>, Tokens> {
    let (mut lhs, mut outer_tokens) = factor(tokens)?;
    loop {
        let (op, tokens) = match mul(outer_tokens).or(div(outer_tokens)) {
            Ok(val) => val,
            Err(_) => break 
        };
        let (rhs, tokens) = factor(tokens)?;
        outer_tokens = tokens;
        lhs = BinOp::new(op, lhs, rhs);
    }
    Ok((lhs, outer_tokens))
}

fn parens(tokens: Tokens) -> Result<Box<Node>, Tokens> {
    let (_, tokens) = lparens(tokens)?;
    let (val, tokens) = expr(tokens)?;
    let (_, tokens) = rparens(tokens)?;
    Ok((val, tokens))
}

fn factor(tokens: Tokens) -> Result<Box<Node>, Tokens> {
    unary(tokens).or(integer(tokens)).or(parens(tokens))
}

fn integer(tokens: Tokens) -> Result<Box<Node>, Tokens> {
    match tokens.first() {
        Some(&Token::Integer(val)) => Ok((Leaf::new(val), pop(tokens))),
        _ => Err(ParseError::Incomplete("Expected integer".to_owned())),
    }
}

fn unary(tokens: Tokens) -> Result<Box<Node>, Tokens> {
    let (op, tokens) = add(tokens).or(sub(tokens))?;
    let (val, tokens) = factor(tokens)?;
    Ok((UnaryOp::new(op, val), tokens))
}

fn mul(tokens: Tokens) -> Result<BinOperator, Tokens> {
    match tokens.first() {
        Some(&Token::Mult) => Ok((BinOperator::Mul, pop(tokens))),
        _ => Err(ParseError::Incomplete("Expected Mul".to_owned()))
    }
}

fn div(tokens: Tokens) -> Result<BinOperator, Tokens> {
    match tokens.first() {
        Some(&Token::Div) => Ok((BinOperator::Div, pop(tokens))),
        _ => Err(ParseError::Incomplete("Expected Div".to_owned()))
    }
}

fn add(tokens: Tokens) -> Result<UnaryOperator, Tokens> {
    match tokens.first() {
        Some(&Token::Plus) => Ok((UnaryOperator::Positive, pop(tokens))),
        _ => Err(ParseError::Incomplete("Expected Add".to_owned()))
    }
}

fn sub(tokens: Tokens) -> Result<UnaryOperator, Tokens> {
    match tokens.first() {
        Some(&Token::Minus) => Ok((UnaryOperator::Negative, pop(tokens))),
        _ => Err(ParseError::Incomplete("Expected Sub".to_owned()))
    } 
}

fn lparens(tokens: Tokens) -> Result<(), Tokens> {
    match tokens.first() {
        Some(&Token::LParens) => Ok(((), pop(tokens))),
        _ => Err(ParseError::Incomplete("Expected Left Parens".to_owned()))
    }
}

fn rparens(tokens: Tokens) -> Result<(), Tokens> {
    match tokens.first() {
        Some(&Token::RParens) => Ok(((), pop(tokens))),
        _ => Err(ParseError::Incomplete("Expected Right Parens".to_owned()))
    }
}

fn pop(tokens: Tokens) -> Tokens { &tokens[1..] }