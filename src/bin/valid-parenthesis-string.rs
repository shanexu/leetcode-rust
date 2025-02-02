fn main() {
    assert_eq!(Solution::check_valid_string("()".to_string()), true);
    assert_eq!(Solution::check_valid_string("(*)".to_string()), true);
}

struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let s = s.as_bytes();
        let mut open = 0;
        for &b in s {
            if b == b'(' {
                open += 1;
            } else if b == b'*' {
                open += 1;
            } else if open > 0 {
                open -= 1;
            } else {
                return false;
            }
        }
        let mut close = 0;
        for &b in s.iter().rev() {
            if b == b')' {
                close += 1;
            } else if b == b'*' {
                close += 1;
            } else if close > 0 {
                close -= 1;
            } else {
                return false;
            }
        }
        true
    }
}
