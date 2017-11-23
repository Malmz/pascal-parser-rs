use super::super::{Leaf, BinOp, BinOperator};
use super::AstVisitor;

pub struct Interpreter {
    store: String,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
             store: String::new(),
        }
    }

    pub fn result(&self) -> &String {
        &self.store
    }
}

impl AstVisitor for Interpreter {
    fn visit_leaf(&mut self, node: &Leaf<i32>) {
        self.store.push_str(&node.val().to_string());
        self.store.push(' ');
    }
    fn visit_binop(&mut self, node: &BinOp) {
        match node.op() {
            BinOperator::Add => self.store.push_str("(+ "),
            BinOperator::Sub => self.store.push_str("(- "),
            BinOperator::Mul => self.store.push_str("(* "),
            BinOperator::Div => self.store.push_str("(/ "),
        }
        node.left().accept(self);
        node.right().accept(self);
        self.store.push_str(") ")
    }
}