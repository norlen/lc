#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

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
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        // There are two cases to consider, assuming we have created an array doing
        // an in-order walk.
        //   1. A_i > A_i+1 only, then A_i > A_i+1 have been swapped.
        //   2. A_i > A_i+1 and A_j > A_j+1 then A_i and A_j+1 have been swapped.
        // But let's do that all in-place instead and only store references to the
        // nodes that are wrong.
        let mut fix = Vec::new();

        let mut current = root.clone();
        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;

        let mut s = Vec::new();
        while current.is_some() || !s.is_empty() {
            while let Some(c) = current {
                s.push(c.clone());
                current = c.borrow().left.clone();
            }

            let n = s.pop().unwrap();
            if prev.is_some() && prev.as_ref().unwrap().borrow().val >= n.borrow().val {
                fix.push((prev.unwrap().clone(), n.clone()));
            }
            prev = Some(n.clone());
            current = n.borrow().right.clone();
        }

        if fix.len() == 1 {
            let (i, j) = fix[0].clone();
            std::mem::swap(&mut i.borrow_mut().val, &mut j.borrow_mut().val);
        } else if fix.len() == 2 {
            let i = fix[0].0.clone();
            let j = fix[1].1.clone();
            std::mem::swap(&mut i.borrow_mut().val, &mut j.borrow_mut().val);
        }
    }
}

fn main() {}
