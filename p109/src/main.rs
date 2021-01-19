#![allow(dead_code)]

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

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

struct Solution {}

impl Solution {
    pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut a = vec![];
        while let Some(h) = head {
            a.push(h.val);
            head = h.next;
        }

        Self::create_tree(a.as_slice())
    }

    fn create_tree(arr: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if arr.len() == 0 {
            return None;
        }
        let m = arr.len() / 2;
        let node = Rc::new(RefCell::new(TreeNode::new(arr[m])));
        node.borrow_mut().left = Self::create_tree(&arr[0..m]);
        node.borrow_mut().right = Self::create_tree(&arr[m + 1..]);
        Some(node)
    }

    pub fn sorted_list_to_bst_variant(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let list_size = {
            let mut s = 0usize;
            let mut h = head.clone();
            while let Some(c) = h {
                s += 1;
                h = c.next;
            }
            s
        };

        Self::ctree(&mut head, 0, list_size)
    }

    fn ctree(
        head: &mut Option<Box<ListNode>>,
        s: usize,
        e: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if s >= e {
            return None;
        }

        let m = s + (e - s) / 2;
        let left = Self::ctree(head, s, m);

        let node = Rc::new(RefCell::new(TreeNode::new(head.as_ref().unwrap().val)));
        *head = head.as_ref().unwrap().next.clone();

        node.borrow_mut().left = left;
        node.borrow_mut().right = Self::ctree(head, m + 1, e);

        Some(node)
    }
}

fn main() {}
