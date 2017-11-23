pub mod expression;
pub mod rpn;
pub mod lisp_style;

use super::{BinOp, Leaf, UnaryOp};

#[allow(unused_variables)]
pub trait AstVisitor {
    fn visit_leaf(&mut self, node: &Leaf<i32>) {}
    fn visit_binop(&mut self, node: &BinOp) {}
    fn visit_unaryop(&mut self, node: &UnaryOp) {}
}