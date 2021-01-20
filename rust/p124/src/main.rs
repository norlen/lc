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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::helper(root.clone()).1
    }

    // Return (current, max)
    fn helper(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        use std::cmp::max;
        if let Some(root) = root {
            let val = root.borrow().val;

            let (left_sum, left_max) = Self::helper(root.borrow().left.clone());
            let (right_sum, right_max) = Self::helper(root.borrow().right.clone());

            // Three cases to consider for current value:
            // 1. Only current node's value.
            // 2. Left sum + value
            // 3. Right sum + value
            // Only one of the children can be with current!
            let current = max(val, max(left_sum + val, right_sum + val));

            // Max has to be separate, if only a subtree has a maximum value, we cannot add that with a current
            // value up the recursion. But for max, we have to consider that both children are part of the path.
            let max = max(max(current, left_sum + right_sum + val), max(left_max, right_max));
            (current, max)
        } else {
            (0, std::i32::MIN) // Base case.
        }
    }
}

fn main() {}
