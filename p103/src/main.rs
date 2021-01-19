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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let root = match root {
            Some(root) => root,
            None => return vec![]
        };
        let mut q = VecDeque::new();
        q.push_back(root);

        let mut zigzag = 0;
        let mut ret = vec![];
        while !q.is_empty() {
            let current_level = q.len();
            let mut level = vec![];
            for _ in 0..current_level {
                let n = q.pop_front().unwrap();
                let n = n.borrow();
                level.push(n.val);

                if let Some(left) = n.left.clone() {
                    q.push_back(left);
                }
                if let Some(right) = n.right.clone() {
                    q.push_back(right);
                }
            }
            if zigzag % 2 == 1 {
                level.reverse();
            }
            zigzag += 1;
            ret.push(level);
        }
        ret
    }
}

fn main() {}
