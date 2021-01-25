#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut current = Vec::new();
        Self::backtrack(k, n, 1, &mut current, &mut result);
        result
    }

    fn backtrack(
        nums_left: i32,
        remaining: i32,
        start: i32,
        current: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if nums_left == 0 || remaining < 0 {
            if remaining == 0 {
                result.push(current.clone());
            }
        }

        for i in start..=9 {
            current.push(i);
            Self::backtrack(nums_left - 1, remaining - i, i + 1, current, result);
            current.pop();
        }
    }
}

fn main() {}
