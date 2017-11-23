use super::Node;
use super::UnaryOperator;
use super::AstVisitor;
use std::fmt::Display;
use std::fmt;

#[derive(Debug)]
pub struct UnaryOp {
    op: UnaryOperator,
    val: Box<Node>,
}

impl UnaryOp {
    pub fn new(op: UnaryOperator, val: Box<Node>) -> Box<Self> {
        Box::new(Self {
            op: op,
            val: val,
        })
    }

    pub fn op(&self) -> UnaryOperator {
        self.op
    }

    pub fn val(&self) -> &Node {
        self.val.as_ref()
    }
}

impl Node for UnaryOp {
    fn accept(&self, visitor: &mut AstVisitor) {
        visitor.visit_unaryop(self);
    }
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}({})", self.op, self.val)
    }
}