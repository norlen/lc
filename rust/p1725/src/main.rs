#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        // Need the highest dimensions for a square, i.e. the lowest of each one.
        let max_s = rectangles
            .iter()
            .fold(0, |m, r| std::cmp::max(m, std::cmp::min(r[0], r[1])));

        rectangles.iter().fold(0, |a, r| {
            if r[0] >= max_s && r[1] >= max_s {
                a + 1
            } else {
                a
            }
        })
    }
}

fn main() {}
