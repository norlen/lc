#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans = Vec::new();
        let mut curr = String::new();
        Self::backtrack(n as usize, (n, n), &mut curr, &mut ans);
        ans
    }

    fn backtrack(n: usize, (open, close): (i32, i32), curr: &mut String, ans: &mut Vec<String>) {
        if curr.len() == (n * 2) {
            ans.push(curr.clone());
        } else {
            if open > 0 {
                curr.push('(');
                Self::backtrack(n, (open - 1, close), curr, ans);
                curr.pop();
            }
            if close > open {
                curr.push(')');
                Self::backtrack(n, (open, close - 1), curr, ans);
                curr.pop();
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
        let ans = vec!["()"];
        assert_eq!(Solution::generate_parenthesis(1), ans);
    }

    #[test]
    fn it_works2() {
        let ans = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
        assert_eq!(Solution::generate_parenthesis(3), ans);
    }
}
