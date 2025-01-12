fn main() {
    println!(
        "{}",
        Solution::min_insertions("(()))(()))()())))".to_string())
    );
    println!("{}", Solution::min_insertions(")))))))".to_string()));
    println!(
        "{}",
        Solution::min_insertions("(((()(()((())))(((()())))()())))(((()(()()((()()))".to_string())
    );
}

struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ans = 0;
        let mut i = 0;
        let n = s.len();
        let mut open = 0;
        while i < s.len() {
            if s[i] == b'(' {
                open += 1;
            } else if s[i] == b')' {
                if i < n - 1 && s[i + 1] == b')' {
                    if open > 0 {
                        open -= 1;
                        i += 1;
                    } else {
                        ans += 1;
                        i += 1;
                    }
                } else {
                    if open > 0 {
                        open -= 1;
                        ans += 1;
                    } else {
                        ans += 2;
                    }
                }
            }
            i += 1;
        }
        ans + open * 2
    }
}
