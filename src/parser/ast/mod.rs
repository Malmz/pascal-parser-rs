pub mod visitors;
pub mod leaf;
pub mod binop;
pub mod unaryop;

pub use self::leaf::Leaf;
pub use self::binop::BinOp;
pub use self::unaryop::UnaryOp;

use std::fmt::Debug;
use std::fmt::Display;
use self::visitors::AstVisitor;


pub trait Node: Debug + Display {
    fn accept(&self, visitor: &mut AstVisitor);
}

#[derive(Clone, Copy, Debug)]
pub enum BinOperator {
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Clone, Copy, Debug)]
pub enum UnaryOperator {
    Positive,
    Negative,
}

impl UnaryOperator {
    pub fn to_bin(self) -> BinOperator {
        match self {
            UnaryOperator::Positive => BinOperator::Add,
            UnaryOperator::Negative => BinOperator::Sub,
        }
    }
}