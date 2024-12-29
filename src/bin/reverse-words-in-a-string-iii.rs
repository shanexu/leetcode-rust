fn main() {
    println!("{}", Solution::reverse_words("Let's take LeetCode contest".to_string()));
}

struct Solution;

impl Solution {
    pub fn reverse_words(mut s: String) -> String {
        let bs = unsafe { s.as_bytes_mut() };
        let mut i = 0;
        for j in 0..bs.len() {
            if bs[j] == b' ' {
                let chunk = &mut bs[i..j];
                chunk.reverse();
                i = j + 1;
            }
        }
        let chunk = &mut bs[i..];
        chunk.reverse();
        s
    }
}