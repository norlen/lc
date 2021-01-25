//! [90. Subsets II](https://leetcode.com/problems/subsets-ii/)
#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut current = Vec::new();
        let mut result = Vec::new();
        Self::backtrack(nums.as_slice(), &mut current, &mut result);
        result
    }

    fn backtrack(nums: &[i32], current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        result.push(current.clone());

        let mut previous = None;
        for (i, n) in nums.iter().copied().enumerate() {
            if let Some(m) = previous {
                if m == n {
                    continue;
                }
            }
            current.push(n);
            Self::backtrack( &nums[i+1..], current, result);
            current.pop();

            previous = Some(n);
        }
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec![1,2,2];
        let answer = vec![vec![],vec![1],vec![1,2],vec![1,2,2],vec![2],vec![2,2]];
        let solution = Solution::subsets_with_dup(input);
        assert_eq!(solution.len(), answer.len());
        for a in answer {
            assert_eq!(solution.contains(&a), true);
        }
    }
}