fn main() {}

struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        fn helper(i: usize, s: &str, dict: &[String], memo: &mut Vec<i8>) -> bool {
            if i == s.len() {
                return true;
            }
            if memo[i] != -1 {
                return memo[i] != 0;
            }
            let ts = &s[i..];
            for segment in dict {
                if ts.starts_with(segment) {
                    if helper(i + segment.len(), s, dict, memo) {
                        memo[i] = 1;
                        return true;
                    }
                }
            }
            memo[i] = 0;
            false
        }
        let mut memo = vec![-1; s.len()];
        helper(0, &s, &word_dict, &mut memo)
    }
}