//! [37. Sudoku Solver](https://leetcode.com/problems/sudoku-solver/)
#![allow(dead_code)]

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut b = Board::new();
        let mut empty_tiles = Vec::new();
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    empty_tiles.push((i, j));
                } else {
                    let n = (board[i][j].to_digit(10).unwrap() - 1) as usize;
                    b.set(i, j, n, true);
                }
            }
        }
        Self::solve(board, empty_tiles.as_slice(), &mut b);
    }

    fn solve(board: &mut Vec<Vec<char>>, tiles: &[(usize, usize)], b: &mut Board) -> bool {
        if tiles.len() == 0 {
            return true;
        }

        let (i, j) = tiles[0];
        for n in 0..9 {
            if b.can_place(i, j, n) {
                b.set(i, j, n, true);
                if Self::solve(board, &tiles[1..], b) {
                    board[i][j] = ('1' as u8 + n as u8) as char;
                    return true;
                }
                b.set(i, j, n, false);
            }
        }
        false
    }

    pub fn solve_sudoku_slow(board: &mut Vec<Vec<char>>) {
        let mut needs_fixing = Vec::new();
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    needs_fixing.push((i, j));
                }
            }
        }
        Self::solve_slow(board, needs_fixing.as_slice());
    }

    fn solve_slow(board: &mut Vec<Vec<char>>, tiles: &[(usize, usize)]) -> bool {
        if !Self::check_board(board) {
            return false;
        } else if tiles.len() == 0 {
            return true;
        }
        let nums = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

        let (i, j) = tiles[0];
        for n in nums.iter().copied() {
            board[i][j] = n;
            if Self::solve_slow(board, &tiles[1..]) {
                return true;
            }
        }
        board[i][j] = '.';
        false
    }

    fn check_board(board: &Vec<Vec<char>>) -> bool {
        let mut m = HashMap::new();
        // Check rows.
        for row in board.iter() {
            for v in row.iter() {
                if *v != '.' && m.contains_key(v) {
                    return false;
                }
                m.insert(*v, true);
            }
            m.clear();
        }

        // Check columns.
        for col in 0..board.len() {
            for v in 0..board.len() {
                if board[v][col] != '.' && m.contains_key(&board[v][col]) {
                    return false;
                }
                m.insert(board[v][col], true);
            }
            m.clear();
        }

        // Check 3x3 grids, of which there are nine.
        for grid in 0..9 {
            for row in 0..3 {
                for col in 0..3 {
                    let i = row + (grid / 3) * 3;
                    let j = col + (grid % 3) * 3;
                    if board[i][j] != '.' && m.contains_key(&board[i][j]) {
                        return false;
                    }
                    m.insert(board[i][j], true);
                }
            }
            m.clear();
        }

        true
    }
}

struct Board {
    rows: Vec<Vec<bool>>,
    cols: Vec<Vec<bool>>,
    boxes: Vec<Vec<bool>>,
}

impl Board {
    fn new() -> Self {
        Self {
            rows: vec![vec![false; 9]; 9],
            cols: vec![vec![false; 9]; 9],
            boxes: vec![vec![false; 9]; 9],
        }
    }

    fn can_place(&self, i: usize, j: usize, n: usize) -> bool {
        !self.rows[i][n] && !self.cols[j][n] && !self.boxes[Self::box_idx(i, j)][n]
    }

    fn set(&mut self, i: usize, j: usize, n: usize, val: bool) {
        self.rows[i][n] = val;
        self.cols[j][n] = val;
        self.boxes[Self::box_idx(i, j)][n] = val;
    }

    fn box_idx(i: usize, j: usize) -> usize {
        (i / 3) * 3 + j / 3
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_board() {
        let board = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];
        assert_eq!(Solution::check_board(&board), true);
    }

    #[test]
    fn check_board2() {
        let board = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '7', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(Solution::check_board(&board), true);
    }

    #[test]
    fn it_works() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let answer = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];
        Solution::solve_sudoku(&mut board);
        assert_eq!(board, answer);
    }
}
