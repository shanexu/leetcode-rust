fn main() {
    println!(
        "{}",
        Solution::min_remove_to_make_valid("lee(t(c)o)de)".to_string())
    );
}

struct Solution;

impl Solution {
    pub fn min_remove_to_make_valid(mut s: String) -> String {
        let s = unsafe { s.as_bytes_mut() };
        let n = s.len();
        let mut count = 0;
        for i in (0..n).rev() {
            let b = s[i];
            if b == b')' {
                count += 1;
            } else if b == b'(' {
                if count > 0 {
                    count -= 1;
                } else {
                    s[i] = 0;
                }
            }
        }
        let mut ans = Vec::with_capacity(n);
        count = 0;
        for i in 0..n {
            let b = s[i];
            if b == b'(' {
                count += 1;
                ans.push(b);
            } else if b == b')' {
                if count > 0 {
                    count -= 1;
                    ans.push(b);
                }
            } else if b != 0 {
                ans.push(b);
            }
        }
        String::from_utf8(ans).unwrap()
    }
}
