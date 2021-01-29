//! [51. N-Queens](https://leetcode.com/problems/n-queens/)
#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut solutions = Vec::new();
        let mut board = Board::new(n as usize);
        Self::solve(&mut board, n as usize, &mut solutions);
        solutions
    }

    fn solve(board: &mut Board, to_place: usize, solutions: &mut Vec<Vec<String>>) {
        if to_place == 0 {
            solutions.push(board.sol.clone());
            return;
        }

        let i = board.size - to_place;
        for j in 0..board.size {
            if board.can_place(i, j) {
                board.set(i, j, true);
                Self::solve(board, to_place - 1, solutions);
                board.set(i, j, false);
            }
        }
    }
}

struct Board {
    size: usize,
    sol: Vec<String>,
    rows: Vec<bool>,
    cols: Vec<bool>,
    diag: Vec<bool>,
    antidiag: Vec<bool>,
}

impl Board {
    fn new(n: usize) -> Self {
        let s = unsafe { String::from_utf8_unchecked(vec!['.' as u8; n]) };
        
        Self {
            size: n,
            sol: vec![s.clone(); n],
            rows: vec![false; n],
            cols: vec![false; n],
            diag: vec![false; 2*n - 1],
            antidiag: vec![false; 2*n - 1],
        }
    }

    fn can_place(&self, i: usize, j: usize) -> bool {
        !(self.rows[i] || self.cols[j] || self.diag[i + j] || self.antidiag[self.size + i - j - 1])
    }

    fn set(&mut self, i: usize, j: usize, val: bool) {
        unsafe {
            self.sol[i].as_bytes_mut()[j] = if val { 'Q' as u8 } else { '.' as u8 };
        }

        self.rows[i] = val;
        self.cols[j] = val;
        self.diag[i + j] = val;
        self.antidiag[self.size + i - j - 1] = val;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ans = vec![vec![".Q..".to_owned(),"...Q".to_owned(),"Q...".to_owned(),"..Q.".to_owned()],vec!["..Q.".to_owned(),"Q...".to_owned(),"...Q".to_owned(),".Q..".to_owned()]];
        let sol = Solution::solve_n_queens(4);
        assert_eq!(sol.len(), ans.len());
        for a in ans {
            assert!(sol.contains(&a));
        }
    }
}
