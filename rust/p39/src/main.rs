#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut curr = Vec::new();
        Solution::backtrack(candidates.as_slice(), target, &mut curr, &mut res);
        res
    }

    fn backtrack(candidates: &[i32], target: i32, curr: &mut Vec<i32>, results: &mut Vec<Vec<i32>>) {
        if target == 0 {
            results.push(curr.clone());
        } else if target < 0 {
            return;
        }

        for (i, c) in candidates.iter().copied().enumerate() {
            curr.push(c);
            Solution::backtrack(&candidates[i..], target - c, curr, results);
            curr.pop();
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
        let cands = vec![2,3,6,7];
        let target = 7;
        let ans = vec![vec![2,2,3], vec![7]];
        let sol = Solution::combination_sum(cands, target);
        assert_eq!(sol.len(), ans.len());
        for mut s in sol {
            s.sort();
            assert_eq!(ans.contains(&s), true);
        }
    }
}