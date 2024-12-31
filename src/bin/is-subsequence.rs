fn main() {
    println!(
        "{}",
        Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string())
    );
    println!(
        "{}",
        Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string())
    );
}

struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s = s.as_bytes();
        let mut t = t.as_bytes();
        while s.len() > 0 {
            if t.len() < s.len() {
                return false;
            }
            let c = s[0];
            if t[0] == c {
                s = &s[1..];
            }
            t = &t[1..];
        }
        true
    }
}
