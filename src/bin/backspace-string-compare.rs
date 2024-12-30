fn main() {
    println!(
        "{}",
        Solution::backspace_compare("#abc##".to_string(), "abc###a".to_string())
    );
}

/// 一开始丧心病狂地给人家方法签名都改了 ```pub fn backspace_compare(mut s: String, mut t: String)```
/// 用原来的空间就不算有空间开销了。后来想到其实可以直接反向比较，从字符串结尾比较，问题就解决了。这题目应该加个
/// 限制条件，不允许修改原始字符串。
struct Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut s_i = s.len();
        let mut t_i = t.len();
        let mut s_b;
        let mut t_b;
        loop {
            (s_i, s_b) = prev(s_i, s);
            (t_i, t_b) = prev(t_i, t);
            if s_b != t_b {
                return false;
            }
            if s_b == 0 && t_b == 0 {
                break;
            }
        }
        true
    }
}

#[inline]
fn prev(mut i: usize, s: &[u8]) -> (usize, u8) {
    let mut sharp_count = 0;
    while i > 0 {
        i -= 1;
        let b = s[i];
        if sharp_count > 0 {
            if b == b'#' {
                sharp_count += 1;
            } else {
                sharp_count -= 1;
            }
        } else {
            if b == b'#' {
                sharp_count += 1;
            } else {
                return (i, b);
            }
        }
    }
    (0, 0)
}
