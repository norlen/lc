#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut current = Vec::new();
        let mut result = Vec::new();
        Self::backtrack(nums.as_slice(), &mut current, &mut result);
        result
    }

    fn backtrack(nums: &[i32], current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        result.push(current.clone());
        for (i, n) in nums.iter().copied().enumerate() {
            current.push(n);
            Self::backtrack(&nums[i+1..], current, result);
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
        let input = vec![1, 2, 3];
        let answer = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        let solution = Solution::subsets(input);
        assert_eq!(solution.len(), answer.len());
        for a in answer {
            assert_eq!(solution.contains(&a), true);
        }
    }
}
