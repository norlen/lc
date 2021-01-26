//! [40. Combination Sum II](https://leetcode.com/problems/combination-sum-ii/)
#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut result = Vec::new();
        let mut current = Vec::new();
        Self::backtrack(candidates.as_slice(), target, &mut current, &mut result);
        result
    }

    fn backtrack(candidates: &[i32], remaining: i32, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if remaining == 0 {
            result.push(current.clone());
            return;
        } else if remaining < 0 {
            return;
        }

        let mut prev = None;
        for (i, c) in candidates.iter().copied().enumerate() {
            if let Some(p) = prev {
                if p == c {
                    continue;
                }
            }
            prev = Some(c);
            
            // Early stopping, as candidates are sorted this is possible.
            if remaining < c {
                return;
            }

            current.push(c);
            Self::backtrack(&candidates[i+1..], remaining - c, current, result);
            current.pop();
        }
    }
}

fn main() {}