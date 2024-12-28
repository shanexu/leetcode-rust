fn main() {
    println!("{}", Solution::min_changes(String::from("1001")));
}

struct Solution;

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let bs = s.as_bytes();
        let mut count = 0;
        for i in (0..bs.len()).step_by(2) {
            count += (bs[i] ^ bs[i + 1]) as i32
        }
        count
    }
}
