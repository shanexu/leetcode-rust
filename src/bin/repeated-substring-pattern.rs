fn main() {
    assert!(Solution::repeated_substring_pattern("abcabc".to_string()));
    assert!(!Solution::repeated_substring_pattern("abc".to_string()));
    assert!(!Solution::repeated_substring_pattern("aba".to_string()));
    assert!(Solution::repeated_substring_pattern(
        "abaababaab".to_string()
    ));
}

struct Solution;

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let s = s.as_bytes();
        let n = s.len();
        let mut lps = vec![0; n];
        let mut i = 1;
        let mut l = 0;
        while i < n {
            if s[i] == s[l] {
                l += 1;
                lps[i] = l;
                i += 1;
            } else {
                if l == 0 {
                    i += 1;
                } else {
                    l = lps[l - 1];
                }
            }
        }
        if l == 0 {
            false
        } else {
            i % (i - l) == 0
        }
    }
}
