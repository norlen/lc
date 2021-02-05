//! [23. Merge k Sorted Lists](https://leetcode.com/problems/merge-k-sorted-lists/)
#![allow(dead_code)]

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

use std::collections::BinaryHeap;
use std::cmp::Ordering;

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();

        for root in lists {
            if let Some(root) = root {
                heap.push(root);
            }
        }

        let mut root = match heap.pop() {
            Some(n) => n,
            None => return None,
        };
        if let Some(next) = root.next.take() {
            heap.push(next);
        }

        // All pointers are owned, so current has to borrowed.
        let mut current: &mut ListNode = root.as_mut();

        while let Some(mut node) = heap.pop() {
            // Current is the tail of our resulting linked list, we want to set the next pointer to
            // the next value which is node. Then we want to add the node's next pointer to the heap.
            //
            // Before: root -> ... -> current -> nil
            //         node -> next -> ...
            // After : root -> ... -> current -> node -> nil
            //         next -> ...

            // Replace next pointer with none for our tail in the final output while adding
            // it, if it exists, to the heap.
            if let Some(to_add) = node.next.take() {
                heap.push(to_add);
            }

            current.next = Some(node);
            current = current.next.as_mut().unwrap();
        }

        Some(root)
    }
    
}

fn main() {}
