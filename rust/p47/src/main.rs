//! [47. Permutations II](https://leetcode.com/problems/permutations-ii/)
#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut result = Vec::new();
        Self::backtrack(&mut nums, 0, &mut result);
        result
    }

    fn backtrack(nums: &mut Vec<i32>, first: usize, result: &mut Vec<Vec<i32>>) {
        if first == nums.len() {
            if !result.contains(nums) {
                result.push(nums.clone());
            }
            return;
        }

        let end = nums.len();
        for i in first..end {
            if (i-first) > 0 && nums[i] == nums[i-1] {
                continue;
            }
            nums.swap(first, i);
            Self::backtrack(nums, first + 1, result);
            nums.swap(first, i);
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, 1, 2];
        let answer = vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]];
        let solution = Solution::permute_unique(nums);
        assert_eq!(solution.len(), answer.len());
        for a in answer {
            assert_eq!(solution.contains(&a), true);
        }
    }

    #[test]
    fn it_works2() {
        let nums = vec![1,2,3];
        let answer = vec![vec![1,2,3],vec![1,3,2],vec![2,1,3],vec![2,3,1],vec![3,1,2],vec![3,2,1]];
        let solution = Solution::permute_unique(nums);
        assert_eq!(solution.len(), answer.len());
        for a in answer {
            assert_eq!(solution.contains(&a), true);
        }
    }

    #[test]
    fn it_works3() {
        let nums = vec![0,1,0,0,9];
        let answer = vec![vec![0,0,0,1,9],vec![0,0,0,9,1],vec![0,0,1,0,9],vec![0,0,1,9,0],vec![0,0,9,0,1],vec![0,0,9,1,0],vec![0,1,0,0,9],vec![0,1,0,9,0],vec![0,1,9,0,0],vec![0,9,0,0,1],vec![0,9,0,1,0],vec![0,9,1,0,0],vec![1,0,0,0,9],vec![1,0,0,9,0],vec![1,0,9,0,0],vec![1,9,0,0,0],vec![9,0,0,0,1],vec![9,0,0,1,0],vec![9,0,1,0,0],vec![9,1,0,0,0]];
        let solution = Solution::permute_unique(nums);
        assert_eq!(solution.len(), answer.len());
        for a in answer {
            assert_eq!(solution.contains(&a), true);
        }
    }
}
