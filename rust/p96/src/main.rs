//! [96. Unique Binary Search Trees](https://leetcode.com/problems/unique-binary-search-trees/)
#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut memo = vec![-1; (n + 1) as usize];
        Self::rec(n, &mut memo)
        // Self::dp(n as usize)
        //Self::catalan(n as i64) as i32
    }

    fn rec(n: i32, memo: &mut Vec<i32>) -> i32 {
        if memo[n as usize] != -1 {
            return memo[n as usize];
        }
        if n < 2 {
            return 1;
        }

        let mut num = 0;
        for i in 0..n {
            num += Self::rec(i, memo) * Self::rec(n - i - 1, memo);
        }
        memo.insert(n as usize, num);
        num
    }

    fn dp(n: usize) -> i32 {
        if n <= 2 {
            return n as i32;
        }
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        dp[1] = 1;

        for i in 2..=n {
            for j in 0..i {
                dp[i] += dp[j] * dp[i - j - 1];
            }
        }
        dp[n]
    }

    fn catalan(n: i64) -> i64 {
        // Catalan numbers:
        // C0 = 1
        // Cn+1 = Cn * 2(2n+1)/(n+2)
        let mut ans: i64 = 1;
        for i in 0..n {
            ans = ans * 2 * (2 * i + 1) / (i + 2);
        }
        ans
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_maybe_works1() {
        assert_eq!(Solution::num_trees(1), 1);
    }

    #[test]
    fn it_maybe_works2() {
        assert_eq!(Solution::num_trees(2), 2);
    }

    #[test]
    fn it_maybe_works3() {
        assert_eq!(Solution::num_trees(3), 5);
    }
}
