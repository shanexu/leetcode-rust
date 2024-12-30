fn main() {
    println!("{}", Solution::minimum_steps(String::from("1001")));
}

struct Solution;

impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let mut count: usize = 0;
        let bs = s.as_bytes();
        let mut j = bs.len();
        while j > 0 && bs[j - 1] == b'1' {
            j -= 1;
        }
        for i in (0..j).rev() {
            if bs[i] == b'1' {
                j -= 1;
                count += j - i;
            }
        }
        count as i64
    }
}
