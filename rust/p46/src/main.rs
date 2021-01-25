//! [46. Permutations](https://leetcode.com/problems/permutations/)
#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut current = Vec::new();
        Self::backtrack(nums.as_slice(), &mut current, &mut result);
        result
    }

    fn backtrack(nums: &[i32], current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if current.len() == nums.len() {
            result.push(current.clone());
            return;
        }
        
        for n in nums.iter().copied() {
            if !current.contains(&n) {
                current.push(n);
                Self::backtrack(nums, current, result);
                current.pop();
            }
        }
    }

    pub fn permute2(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let k = nums.len();
        Self::heaps(&mut nums, k, &mut result);
        result
    }

    fn heaps(nums: &mut Vec<i32>, k: usize, result: &mut Vec<Vec<i32>>) {
        if k == 1 {
            result.push(nums.clone());
            return;
        }

        for i in 0..k {
            Self::heaps(nums, k - 1, result);

            if k % 2 == 0 {
                nums.swap(0, k - 1);
            } else {
                nums.swap(i, k - 1);
            }
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, 2, 3];
        let answer = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        let solution = Solution::permute(nums);

        println!("s: {:?}", solution);
        println!("a: {:?}", answer);

        assert_eq!(solution.len(), answer.len());
        for a in answer {
            assert_eq!(solution.contains(&a), true);
        }
    }
}
