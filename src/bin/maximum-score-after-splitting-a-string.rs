fn main() {
    println!("{}", Solution::max_score("011101".to_string()));
    println!("{}", Solution::max_score("00111".to_string()));
    println!("{}", Solution::max_score("1111".to_string()));
}

struct Solution;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ones = 0;
        for &b in s {
            if b == b'1' {
                ones += 1;
            }
        }
        let mut score = i32::MIN;
        let mut left_zeros = 0;
        let mut left_ones = 0;
        for &b in s.iter().take(s.len() - 1) {
            if b == b'0' {
                left_zeros += 1;
            } else {
                left_ones += 1;
            }
            let right_ones = ones - left_ones;
            score = score.max(left_zeros + right_ones)
        }
        score
    }
}
