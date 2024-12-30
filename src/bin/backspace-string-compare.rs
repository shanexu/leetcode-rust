fn main() {
    println!(
        "{}",
        Solution::backspace_compare("#abc##".to_string(), "abc###a".to_string())
    );
}

struct Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut s_i = s.len();
        let mut t_i = t.len();
        let mut s_b: u8 = 0;
        let mut t_b: u8 = 0;
        loop {
            (s_i, s_b) = prev(s_i, s);
            (t_i, t_b) = prev(t_i, t);
            if s_b != t_b {
                return false;
            }
            if s_b == t_b && s_b == 0 {
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
