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

use std::collections::HashMap;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut map = HashMap::new();
        for (i, v) in inorder.iter().copied().enumerate() {
            map.insert(v, i);
        }
        Self::helper(&map, &mut preorder.iter().copied(), 0, preorder.len())
    }

    fn helper(
        m: &HashMap<i32, usize>,
        pre: &mut impl Iterator<Item = i32>,
        start: usize,
        end: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if start >= end {
            return None;
        } else if let Some(val) = pre.next() {
            let node = Rc::new(RefCell::new(TreeNode::new(val)));
            let val_idx = match m.get(&val).copied() {
                Some(v) => v,
                None => return None,
            };
            node.borrow_mut().left = Self::helper(m, pre, start, val_idx);
            node.borrow_mut().right = Self::helper(m, pre, val_idx + 1, end);
            Some(node)
        } else {
            None
        }
    }
}

fn main() {}
