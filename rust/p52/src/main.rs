//! [52. N-Queens II](https://leetcode.com/problems/n-queens-ii/)
#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut total = 0;
        let mut b = Board::new(n as usize);
        Self::solve(&mut b, n as usize, &mut total);
        total
    }

    fn solve(b: &mut Board, remaining: usize, total: &mut i32) {
        if remaining == 0 {
            *total += 1;
            return;
        }

        let i = b.n - remaining;
        for j in 0..b.n {
            if b.can_place(i, j) {
                b.set(i, j, true);
                Self::solve(b, remaining - 1, total);
                b.set(i, j, false);
            }
        }
    }
}

struct Board {
    n: usize,
    row: Vec<bool>,
    col: Vec<bool>,
    d: Vec<bool>,
    ad: Vec<bool>,
}

impl Board {
    fn new(n: usize) -> Self {
        Self {
            n: n,
            row: vec![false; n],
            col: vec![false; n],
            d: vec![false; n * 2 - 1],
            ad: vec![false; n * 2 - 1],
        }
    }

    fn can_place(&self, i: usize, j: usize) -> bool {
        !(self.row[i] || self.col[j] || self.d[i + j] || self.ad[self.n + i - j - 1])
    }

    fn set(&mut self, i: usize, j: usize, v: bool) {
        self.row[i] = v;
        self.col[j] = v;
        self.d[i + j] = v;
        self.ad[self.n + i - j - 1] = v;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ps = [
            (1, 1),
            (2, 0),
            (3, 0),
            (4, 2),
            (5, 10),
            (6, 4),
            (7, 40),
            (8, 92),
            (9, 352),
        ];
        for (n, a) in ps.iter().copied() {
            assert_eq!(Solution::total_n_queens(n), a);
        }
    }
}
