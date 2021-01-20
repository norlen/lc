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
struct Solution;

impl Solution {
    fn preorder(mut root: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<Rc<RefCell<TreeNode>>>) {
        let mut s = Vec::new();
        while let Some(r) = root {
            if let Some(r) = r.borrow().right.clone() {
                s.push(r);
            }

            arr.push(r.clone());

            root = if let Some(l) = r.borrow().left.clone() {
                Some(l.clone())
            } else {
                s.pop()
            };
        }
    }

    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        // Simple solution:
        // 1. Preorder traversal
        // 2. Put all nodes in arr
        // 3. Fix them up.
        let mut a = Vec::new();
        Self::preorder(root.clone(), &mut a);

        for i in 1..a.len() {
            a[i - 1].borrow_mut().right = Some(a[i].clone());
            a[i - 1].borrow_mut().left = None;
        }
    }
}

fn main() {}
