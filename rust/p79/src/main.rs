//! [79. Word Search](https://leetcode.com/problems/word-search/)
#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        for x in 0..board.len() {
            for y in 0..board[0].len() {
                if Self::solve(&mut board, (x as i32, y as i32), word.as_bytes()) {
                    return true;
                }
            }
        }
        false
    }

    fn solve(board: &mut Vec<Vec<char>>, (x, y): (i32, i32), word: &[u8]) -> bool {
        if word.len() == 0 {
            return true;
        }
        if x < 0
            || y < 0
            || x >= board.len() as i32
            || y >= board[0].len() as i32
            || board[x as usize][y as usize] as u8 != word[0]
        {
            return false;
        }

        board[x as usize][y as usize] = '#';
        let offsets = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in offsets.iter().copied() {
            if Self::solve(board, (x + dx, y + dy), &word[1..]) {
                return true;
            }
        }
        board[x as usize][y as usize] = word[0] as char;
        false
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let b = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert_eq!(Solution::exist(b, "ABCCED".to_owned()), true);
    }

    #[test]
    fn no_already_used_positions() {
        let b = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert_eq!(Solution::exist(b, "ABCB".to_owned()), false);
    }
}
