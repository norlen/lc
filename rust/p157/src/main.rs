//! [157. Read N Characters Given Read4](https://leetcode.com/problems/read-n-characters-given-read4/)
#![allow(dead_code)]

struct Solution;

/**
 * The read4 API is defined as.
 *     fn read4(&self,buf4: &mut [char]) -> i32;
 * You can call it using self.read4(buf4)
 */
impl Solution {
    pub fn read(&self, buf: &mut [char], n: i32) -> i32 {
        let mut i = 0;
        let mut scratch = [0 as char; 4];

        loop {
            let bytes_read = self.read4(&mut scratch);

            // Copy either the amount of bytes read, or what we have left until `n` is hit.
            let end_idx = std::cmp::min(bytes_read as usize, n as usize - i);

            // Copy over bytes from scratch.
            for j in 0..end_idx {
                buf[i] = scratch[j];
                i += 1;
            }

            // Done if we copied less than 4 bytes.
            if end_idx < 4 {
                break;
            }
        }
        i as i32
    }

    // If we're allowed to assume that the length of buf is 4*k.
    fn read2(&self, buf: &mut [char], n: i32) -> i32 {
        let mut i = 0;
        while i < n {
            let bytes_read = self.read4(&mut buf[i as usize..]);
            i += bytes_read;
            if bytes_read < 4 {
                break;
            }
        }
        std::cmp::min(i, n)
    }

    fn read4(&self, _buf: &mut [char]) -> i32 {
        unimplemented!()
    }
}

fn main() {}
