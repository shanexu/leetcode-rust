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
        let mut stack = vec![];
        let mut ans = 0;
        let mut i = 0;
        let n = s.len();
        while i < s.len() {
            if s[i] == b')' {
                if i < n - 1 && s[i + 1] == b')' {
                    match stack.last() {
                        Some(&b'(') => {
                            stack.pop();
                        }
                        _ => stack.push(b']'),
                    }
                    i += 1;
                } else {
                    stack.push(s[i]);
                }
            } else {
                stack.push(s[i]);
            }
            i += 1;
        }
        let mut open = 0;
        for &b in stack.iter() {
            if b == b'(' {
                open += 1;
            } else if b == b')' {
                ans += 1;
                if open > 0 {
                    open -= 1;
                } else {
                    ans += 1;
                }
            } else if b == b']' {
                if open > 0 {
                    open -= 1;
                } else {
                    ans += 1;
                }
            }
        }
        ans + open * 2
    }
}
