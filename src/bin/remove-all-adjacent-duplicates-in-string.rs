fn main() {
    assert_eq!(
        "ca".to_string(),
        Solution::remove_duplicates("abbaca".to_string())
    );
}

struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let s = s.as_bytes();
        let mut stack = vec![0u8; s.len()];
        let mut len = 0;
        for &b in s {
            if len != 0 && stack[len - 1] == b {
                len -= 1
            } else {
                stack[len] = b;
                len += 1
            }
        }
        stack.resize(len, 0);
        String::from_utf8(stack).unwrap()
    }
}
