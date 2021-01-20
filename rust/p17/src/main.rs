#![allow(dead_code)]

struct Solution {}

impl Solution {
    fn letters_from_digit(ch: char) -> Vec<&'static str> {
        match ch {
            '1' => vec![],
            '2' => vec!["a", "b", "c"],
            '3' => vec!["d", "e", "f"],
            '4' => vec!["g", "h", "i"],
            '5' => vec!["j", "k", "l"],
            '6' => vec!["m", "n", "o"],
            '7' => vec!["p", "q", "r", "s"],
            '8' => vec!["t", "u", "v"],
            '9' => vec!["w", "x", "y", "z"],
            _ => panic!(format!("Unsupported character {}", ch)),
        }
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return Vec::new();
        }

        let digit: char = digits.as_bytes()[0] as char;
        if digits.len() == 1 {
            return Self::letters_from_digit(digit)
                .into_iter()
                .map(|s| s.to_owned())
                .collect();
        }

        let combos = Self::letter_combinations(digits[1..].to_owned());
        Self::letters_from_digit(digit)
            .into_iter()
            .flat_map(|l| {
                combos
                    .clone()
                    .into_iter()
                    .map(|s| format!("{}{}", l, s))
                    .collect::<Vec<String>>()
            })
            .collect()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(
            Solution::letter_combinations("".to_owned()),
            Vec::<String>::new()
        );
    }

    #[test]
    fn single_digit() {
        let ans = vec!["a", "b", "c"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        assert_eq!(Solution::letter_combinations("2".to_owned()), ans);
    }

    #[test]
    fn basic() {
        let ans = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        assert_eq!(Solution::letter_combinations("23".to_owned()), ans);
    }

    #[test]
    fn long() {
        let ans = vec![
            "adgj", "adgk", "adgl", "adhj", "adhk", "adhl", "adij", "adik", "adil", "aegj", "aegk",
            "aegl", "aehj", "aehk", "aehl", "aeij", "aeik", "aeil", "afgj", "afgk", "afgl", "afhj",
            "afhk", "afhl", "afij", "afik", "afil", "bdgj", "bdgk", "bdgl", "bdhj", "bdhk", "bdhl",
            "bdij", "bdik", "bdil", "begj", "begk", "begl", "behj", "behk", "behl", "beij", "beik",
            "beil", "bfgj", "bfgk", "bfgl", "bfhj", "bfhk", "bfhl", "bfij", "bfik", "bfil", "cdgj",
            "cdgk", "cdgl", "cdhj", "cdhk", "cdhl", "cdij", "cdik", "cdil", "cegj", "cegk", "cegl",
            "cehj", "cehk", "cehl", "ceij", "ceik", "ceil", "cfgj", "cfgk", "cfgl", "cfhj", "cfhk",
            "cfhl", "cfij", "cfik", "cfil",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
        assert_eq!(Solution::letter_combinations("2345".to_owned()), ans);
    }
}
