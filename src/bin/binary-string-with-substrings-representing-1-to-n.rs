fn main() {
    println!("{}", Solution::query_string("1001".to_string(), 3));
}

struct Solution;

impl Solution {
    pub fn query_string(s: String, n: i32) -> bool {
        for i in (n / 2 + 1)..=n {
            if !s.contains(format!("{:b}", i).as_str()) {
                return false;
            }
        }
        true
    }
}
