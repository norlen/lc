#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

impl Solution {
    pub fn is_valid_bst(mut root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut prev = None;
        let mut s = Vec::new();

        while root.is_some() || !s.is_empty() {
            while let Some(valid_root) = root {
                s.push(valid_root.clone());
                root = valid_root.borrow().left.clone();
            }

            let n = s.pop().unwrap();
            if prev.is_some() && prev.unwrap() >= n.borrow().val {
                return false;
            }
            prev = Some(n.borrow().val);
            root = n.borrow().right.clone();
        }
        true
    }
}

fn main() {}
