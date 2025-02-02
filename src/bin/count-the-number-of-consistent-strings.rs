fn main() {
    assert_eq!(
        Solution::count_consistent_strings(
            "ab".to_string(),
            vec![
                "ad".to_string(),
                "bd".to_string(),
                "aaab".to_string(),
                "baa".to_string(),
                "badab".to_string()
            ]
        ),
        2
    );
    assert_eq!(
        Solution::count_consistent_strings(
            "abc".to_string(),
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "ab".to_string(),
                "ac".to_string(),
                "bc".to_string(),
                "abc".to_string()
            ]
        ),
        7
    );
    assert_eq!(
        Solution::count_consistent_strings(
            "cad".to_string(),
            vec![
                "cc".to_string(),
                "acd".to_string(),
                "b".to_string(),
                "ba".to_string(),
                "bac".to_string(),
                "bad".to_string(),
                "ac".to_string(),
                "d".to_string()
            ]
        ),
        4
    );
}

struct Solution;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut freq = 0u32;
        let allowed = allowed.as_bytes();
        for &b in allowed {
            freq |= 1 << (b - b'a') as u32;
        }
        let mut ans = 0;
        'out: for w in words {
            for &b in w.as_bytes() {
                if freq | (1 << (b - b'a') as u32) != freq {
                    continue 'out;
                }
            }
            ans += 1;
        }
        ans
    }
}
