fn main() {}

struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let s = s.as_bytes();
        let mut freq = [0; 26];
        for &b in s {
            freq[(b - b'a') as usize] += 1;
        }
        for (i, &b) in s.iter().enumerate() {
            if freq[(b - b'a') as usize] == 1 {
                return i as i32;
            }
        }
        -1
    }
}
