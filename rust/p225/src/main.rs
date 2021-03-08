//! [225. Implement Stack using Queues](https://leetcode.com/problems/implement-stack-using-queues/)
#![allow(dead_code)]

use std::collections::VecDeque;

// Implement a last in first out (LIFO) stack using only two queues. The implemented stack should support all the functions of a normal queue (push, top, pop, and empty).
struct MyStack {
    active: usize,
    queues: [VecDeque<i32>; 2],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            active: 0,
            queues: [VecDeque::new(), VecDeque::new()],
        }
    }

    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        self.queues[self.active].push_back(x);
    }

    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        let val = match self.queues[self.active].len() {
            0 => panic!(),
            1 => self.queues[self.active].pop_front().unwrap(),
            _ => {
                self.queue_trickery();
                self.queues[self.active].pop_front().unwrap()
            }
        };
        self.active = if self.active == 0 { 1 } else { 0 };
        val
    }

    /** Get the top element. */
    fn top(&mut self) -> i32 {
        match self.queues[self.active].len() {
            0 => panic!(),
            1 => self.queues[self.active].front().copied().unwrap(),
            _ => {
                self.queue_trickery();
                self.queues[self.active].front().copied().unwrap()
            }
        }
    }

    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        self.queues[0].is_empty() && self.queues[1].is_empty()
    }

    fn queue_trickery(&mut self) {
        let (active, other) = if self.active == 0 { (0, 1) } else { (1, 0) };

        while self.queues[active].len() > 1 {
            let e = self.queues[active].pop_front().unwrap();
            self.queues[other].push_back(e);
        }
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

fn main() {}
