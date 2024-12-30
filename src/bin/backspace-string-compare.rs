fn main() {
    println!(
        "{}",
        Solution::backspace_compare("#abc##".to_string(), "abc###a".to_string())
    );
}

struct Solution;

impl Solution {
    pub fn backspace_compare(mut s: String, mut t: String) -> bool {
        fn process(s: &mut [u8]) -> &[u8] {
            let mut i: usize = 0;
            for j in 0..s.len() {
                if s[j] == b'#' {
                    if i != 0 {
                        i -= 1;
                    }
                } else {
                    s[i] = s[j];
                    i += 1;
                }
            }
            &s[0..i]
        }
        let s = unsafe { s.as_bytes_mut() };
        let t = unsafe { t.as_bytes_mut() };
        process(s) == process(t)
    }
}
