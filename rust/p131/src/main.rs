//! [131. Palindrome Partitioning](https://leetcode.com/problems/palindrome-partitioning/)
#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let mut current = Vec::new();
        Self::backtrack(&s, &mut current, &mut result);
        result
    }

    fn backtrack(s: &str, current: &mut Vec<String>, result: &mut Vec<Vec<String>>) {
        if s.len() == 0 {
            result.push(current.clone());
            return;
        }

        let mut substr = String::new();
        for (i, c) in s.chars().enumerate() {
            substr.push(c);
            if Self::is_palindrome(&substr) {
                current.push(substr.clone());
                Self::backtrack(&s[i+1..], current, result);
                current.pop();
            }
        }
    }

    fn is_palindrome(s: &str) -> bool {
        let s = s.as_bytes();
        let mut i = 0;
        let mut j = s.len();
        while i+1 < j {
            if s[i] != s[j-1] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "aab";
        let ans = vec![
            vec!["a".to_owned(), "a".to_owned(), "b".to_owned()],
            vec!["aa".to_owned(), "b".to_owned()],
        ];
        let sol = Solution::partition(s.to_owned());
        assert_eq!(sol.len(), ans.len());
        for a in ans {
            assert!(sol.contains(&a));
        }
    }
}
