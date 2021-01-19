#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;
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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let imap = {
            let mut map = HashMap::new();
            inorder.iter().enumerate().for_each(|(i, &v)| {
                map.insert(v, i);
            });
            map
        };

        let mut postorder_rev = postorder.iter().copied().rev();
        Self::helper(&imap, 0, inorder.len(), &mut postorder_rev)
    }

    fn helper(
        imap: &HashMap<i32, usize>,
        is: usize,
        ie: usize,
        post: &mut impl Iterator<Item = i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if is >= ie {
            return None;
        }

        if let Some(val) = post.next() {
            let node = Rc::new(RefCell::new(TreeNode::new(val)));
            let i = match imap.get(&val) {
                Some(&v) => v,
                None => return None,
            };

            node.borrow_mut().right = Self::helper(imap, i + 1, ie, post);
            node.borrow_mut().left = Self::helper(imap, is, i, post);
            Some(node)
        } else {
            None
        }
    }
}

fn main() {}
