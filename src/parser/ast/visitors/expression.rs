use super::super::{Leaf, BinOp, BinOperator, UnaryOp, UnaryOperator};
use super::AstVisitor;

pub struct Interpreter {
    store: Vec<i32>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
             store: Vec::new(),
        }
    }

    pub fn result(&self) -> Option<i32> {
        match self.store.len() {
            0 => None,
            n => Some(self.store[n-1])
        }
    }
}

impl AstVisitor for Interpreter {
    fn visit_leaf(&mut self, node: &Leaf<i32>) {
        self.store.push(*node.val())
    }

    fn visit_binop(&mut self, node: &BinOp) {
        node.left().accept(self);
        node.right().accept(self);
        let (rhs, lhs) = (self.store.pop().unwrap(), self.store.pop().unwrap());
        match node.op() {
            BinOperator::Add => self.store.push(lhs + rhs),
            BinOperator::Sub => self.store.push(lhs - rhs),
            BinOperator::Mul => self.store.push(lhs * rhs),
            BinOperator::Div => self.store.push(lhs / rhs),
        }
    }

    fn visit_unaryop(&mut self, node: &UnaryOp) {
        node.val().accept(self);
        let val = self.store.pop().unwrap();
        match node.op() {
            UnaryOperator::Positive => self.store.push(val),
            UnaryOperator::Negative => self.store.push(-val),
        }
    }
}