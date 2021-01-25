//! [77. Combinations](https://leetcode.com/problems/combinations/submissions/)
#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut current = Vec::new();
        Self::backtrack(n, k, &mut current, &mut result);
        result
    }

    fn backtrack(n: i32, k: i32, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if k == 0 {
            result.push(current.clone());
            return;
        }

        let start = current.last().copied().unwrap_or(0);
        for i in start + 1..=n {
            current.push(i);
            Self::backtrack(n, k - 1, current, result);
            current.pop();
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let answer = vec![
            vec![2, 4],
            vec![3, 4],
            vec![2, 3],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
        ];
        let sol = Solution::combine(4, 2);
        assert_eq!(sol.len(), answer.len());
        for a in answer {
            assert_eq!(sol.contains(&a), true);
        }
    }
}
