fn main() {
    println!("{}", Solution::min_remove_to_make_valid("lee(t(c)o)de)".to_string()));
}

struct Solution;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let s = s.as_bytes();
        let mut t = vec![];
        let mut close = 0;
        for &b in s.iter().rev() {
            if b == b')' {
                close += 1;
                t.push(b);
            } else if b == b'(' {
                if close > 0 {
                    close -= 1;
                    t.push(b);
                }
            } else {
                t.push(b);
            }
        }
        let mut ans = vec![];
        let mut open = 0;
        for &b in t.iter().rev() {
            if b == b'(' {
                open += 1;
                ans.push(b);
            } else if b == b')' {
                if open > 0 {
                    open -= 1;
                    ans.push(b);
                }
            } else {
                ans.push(b);
            }
        }
        String::from_utf8(ans).unwrap()
    }
}