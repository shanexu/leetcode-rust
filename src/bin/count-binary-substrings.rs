fn main() {
    assert_eq!(Solution::count_binary_substrings("00110011".to_string()), 6);
    assert_eq!(Solution::count_binary_substrings("10101".to_string()), 4);
    assert_eq!(Solution::count_binary_substrings("00110".to_string()), 3);
    assert_eq!(Solution::count_binary_substrings("00100".to_string()), 2);
    assert_eq!(Solution::count_binary_substrings("0010".to_string()), 2);
    assert_eq!(Solution::count_binary_substrings("001".to_string()), 1);
    assert_eq!(Solution::count_binary_substrings("00".to_string()), 0);
    assert_eq!(Solution::count_binary_substrings("0".to_string()), 0);
    assert_eq!(Solution::count_binary_substrings("".to_string()), 0);
    assert_eq!(Solution::count_binary_substrings("000".to_string()), 0);
    assert_eq!(Solution::count_binary_substrings("0000".to_string()), 0);
    assert_eq!(Solution::count_binary_substrings("00000".to_string()), 0);
    assert_eq!(Solution::count_binary_substrings("000000".to_string()), 0);
    assert_eq!(Solution::count_binary_substrings("0000000".to_string()), 0);
    assert_eq!(Solution::count_binary_substrings("00000000".to_string()), 0);
    assert_eq!(
        Solution::count_binary_substrings("000000000".to_string()),
        0
    );
    assert_eq!(
        Solution::count_binary_substrings("0000000000".to_string()),
        0
    );
    assert_eq!(
        Solution::count_binary_substrings("00000000000".to_string()),
        0
    );
    assert_eq!(
        Solution::count_binary_substrings("000000000000".to_string()),
        0
    );
    assert_eq!(
        Solution::count_binary_substrings("0000000000000".to_string()),
        0
    );
    assert_eq!(
        Solution::count_binary_substrings("00000000000000".to_string()),
        0
    );
    assert_eq!(
        Solution::count_binary_substrings("000000000000000".to_string()),
        0
    );
    assert_eq!(
        Solution::count_binary_substrings("0000000000000000".to_string()),
        0
    );
}

struct Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut prev = 0;
        let mut cur = 1;
        let mut res = 0;
        for i in 1..s.len() {
            if s[i] == s[i - 1] {
                cur += 1;
            } else {
                res += prev.min(cur);
                prev = cur;
                cur = 1;
            }
        }
        res + prev.min(cur)
    }
}
