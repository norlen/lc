#![allow(dead_code)]

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

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut res = Self::p(root, target_sum);
        res.iter_mut().for_each(|a| a.reverse());
        res
    }

    fn p(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        if let Some(root) = root {
            let val = root.borrow().val;

            // Is leaf and target sum with this node is zero
            if root.borrow().left.is_none()
                && root.borrow().right.is_none()
                && target_sum - val == 0
            {
                vec![vec![val]]
            } else {
                let mut left = Self::p(root.borrow().left.clone(), target_sum - val);
                let mut right = Self::p(root.borrow().right.clone(), target_sum - val);

                match (left.len(), right.len()) {
                    (x, y) if x > 0 && y > 0 => {
                        left.append(&mut right);
                        left.iter_mut().for_each(|a| a.push(val));
                    }
                    (x, y) if x > 0 && y == 0 => left.iter_mut().for_each(|a| a.push(val)),
                    (x, y) if x == 0 && y > 0 => right.iter_mut().for_each(|a| a.push(val)),
                    _ => {}
                };

                if left.len() == 0 {
                    right
                } else {
                    left
                }
            }
        } else {
            vec![]
        }
    }
}

fn main() {}
