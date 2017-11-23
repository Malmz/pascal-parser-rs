use super::Node;
use super::visitors::AstVisitor;
use std::fmt::Display;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Leaf<T> {
    val: T
}

impl<T> Leaf<T> {
    pub fn new(val: T) -> Box<Self> {
        Box::new(Self {
            val: val
        })
    }

    pub fn val(&self) -> &T {
        &self.val
    }
}

impl Node for Leaf<i32> {
    fn accept(&self, visitor: &mut AstVisitor) {
        visitor.visit_leaf(self)
    }
}

impl<T> Display for Leaf<T> 
where T: Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}