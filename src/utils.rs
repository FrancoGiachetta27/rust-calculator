use crate::parser::GramItem::{self, Id, Num, Op};

use std::fmt::Debug;

#[derive(Debug)]
pub struct ParseNode {
    root: GramItem,
    children: Vec<Self>,
}

impl ParseNode {
    pub fn new(data: GramItem) -> Self {
        Self {
            root: data,
            children: vec![],
        }
    }

    // pub fn trav_pre_order(&self) {
    //     println!("{:?}", self.root);

    //     if let Some(ref sub) = self.left {
    //         sub.trav_pre_order();
    //     }
    //     if let Some(ref sub) = self.right {
    //         sub.trav_pre_order();
    //     }
    // }

    // pub fn trav_in_order(&self) {
    //     if let Some(ref sub) = self.left {
    //         sub.trav_in_order();
    //     }

    //     println!("{:?}", self.root);

    //     if let Some(ref sub) = self.right {
    //         sub.trav_in_order();
    //     }
    // }
}
