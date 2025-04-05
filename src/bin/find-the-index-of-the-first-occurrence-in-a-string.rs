fn main() {
    println!(
        "{}",
        Solution::str_str("leetcode".to_string(), "leeto".to_string())
    );
}

struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();
        let lps = construct_lps(needle);
        let mut i = 0;
        let mut j = 0;
        let m = haystack.len();
        let n = needle.len();
        while i < m {
            if haystack[i] == needle[j] {
                i += 1;
                j += 1;
                if j == n {
                    return (i - j) as _;
                }
            } else {
                if j == 0 {
                    i += 1;
                } else {
                    j = lps[j - 1];
                }
            }
        }

        return -1;
    }
}

#[inline]
fn construct_lps(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut lps = vec![0; n];
    let mut len = 0;
    let mut i = 1;

    while i < n {
        if s[i] == s[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else {
            if len == 0 {
                i += 1;
            } else {
                len = lps[len - 1];
            }
        }
    }

    lps
}
