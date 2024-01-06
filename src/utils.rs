use crate::parser::GramItem::{self, Id, Num, Op};

use std::fmt::Debug;

#[derive(Debug)]
pub struct ParseTree {
    pub root: GramItem,
    pub left: SubTree,
    pub right: SubTree,
}

type SubTree = Option<Box<ParseTree>>;

impl ParseTree {
    pub fn new(root: GramItem) -> Self {
        Self {
            root,
            left: None,
            right: None,
        }
    }

    pub fn trav_pre_order(&self) {
        println!("{:?}", self.root);

        if let Some(ref sub) = self.left {
            sub.trav_pre_order();
        }
        if let Some(ref sub) = self.right {
            sub.trav_pre_order();
        }
    }

    pub fn trav_in_order(&self) {
        if let Some(ref sub) = self.left {
            sub.trav_in_order();
        }

        println!("{:?}", self.root);

        if let Some(ref sub) = self.right {
            sub.trav_in_order();
        }
    }
}
