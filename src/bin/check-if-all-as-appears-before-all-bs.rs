fn main() {
    println!("{}", Solution::check_string("a".to_string()));
    println!("{}", Solution::check_string("b".to_string()));
    println!("{}", Solution::check_string("".to_string()));
    println!("{}", Solution::check_string("aabb".to_string()));
    println!("{}", Solution::check_string("aba".to_string()));
}

struct Solution;

impl Solution {
    pub fn check_string(s: String) -> bool {
        !s.contains("ba")
    }
}