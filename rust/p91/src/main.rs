//! [91. Decode Ways](https://leetcode.com/problems/decode-ways/)
#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        Self::dps2(&s)
    }

    fn dps2(s: &str) -> i32 {
        if s.as_bytes()[0] == '0' as u8 {
            return 0;
        }
        let mut a = 1;
        let mut b = 1;

        for i in 1..s.len() {
            let mut current = 0;
            if s.as_bytes()[i] != '0' as u8 {
                current = b;
            }

            let n = s[(i - 1)..(i + 1)].parse::<i64>().unwrap();
            if n >= 10 && n <= 26 {
                current += a;
            }
            a = b;
            b = current;
        }
        b
    }

    fn dps(s: &str) -> i32 {
        let mut dp: Vec<i32> = vec![0; s.len() + 1];
        dp[0] = 1;
        dp[1] = if s.as_bytes()[0] == '0' as u8 {
            0 // If invalid.
        } else {
            1
        };

        for i in 2..dp.len() {
            if s.as_bytes()[i - 1] != '0' as u8 {
                dp[i] = dp[i - 1];
            }

            let n = s[(i - 2)..i].parse::<i64>().unwrap();
            if n >= 10 && n <= 26 {
                dp[i] += dp[i - 2];
            }
        }
        *dp.last().unwrap()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "12".to_owned();
        assert_eq!(Solution::num_decodings(s), 2);
    }

    #[test]
    fn it_works2() {
        let s = "226".to_owned();
        assert_eq!(Solution::num_decodings(s), 3);
    }

    #[test]
    fn it_works3() {
        let s = "0".to_owned();
        assert_eq!(Solution::num_decodings(s), 0);
    }

    #[test]
    fn it_works4() {
        let s = "06".to_owned();
        assert_eq!(Solution::num_decodings(s), 0);
    }
}
