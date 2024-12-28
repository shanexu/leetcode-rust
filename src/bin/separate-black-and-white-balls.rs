fn main() {
    println!("{}", Solution::minimum_steps(String::from("111000")));
}

struct Solution;

impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let mut count: usize = 0;
        let bs = s.as_bytes();
        let mut j = bs.len();
        for k in (0..bs.len()).rev() {
            if bs[k] == b'1' {
                j = k;
            } else {
                break;
            }
        }
        for i in (0..j).rev() {
            if bs[i] == b'0' {
                continue;
            }
            if bs[i] == b'1' {
                j -= 1;
                count += j - i;
                continue;
            }
        }
        count as i64
    }
}