//! [206. Reverse Linked List](https://leetcode.com/problems/reverse-linked-list/)
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

struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut previous = match head {
            Some(n) => n,
            None => return None,
        };
        let mut current = match previous.next.take() {
            Some(n) => n,
            None => return Some(previous),
        };

        while let Some(next) = current.next.replace(previous) {
            previous = current;
            current = next;
        }
        Some(current)
    }
}

fn main() {}
