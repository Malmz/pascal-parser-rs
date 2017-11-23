use super::Node;
use super::BinOperator;
use super::AstVisitor;
use std::fmt::Display;
use std::fmt;

#[derive(Debug)]
pub struct BinOp {
    op: BinOperator,
    l: Box<Node>,
    r: Box<Node>
}

impl BinOp {
    pub fn new(op: BinOperator, left: Box<Node>, right: Box<Node>) -> Box<Self> {
        Box::new(Self {
            op: op,
            l: left,
            r: right,
        })
    }

    pub fn op(&self) -> BinOperator {
        self.op
    }

    pub fn left(&self) -> &Node {
        self.l.as_ref()
    } 

    pub fn right(&self) -> &Node {
        self.r.as_ref()
    } 
}

impl Node for BinOp {
    fn accept(&self, visitor: &mut AstVisitor) {
        visitor.visit_binop(self);
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}({}, {})", self.op, self.l, self.r)
    }
}