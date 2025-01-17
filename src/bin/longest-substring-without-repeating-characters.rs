fn main() {
    println!(
        "{}",
        Solution::length_of_longest_substring("abcabcbb".to_string())
    );
    println!(
        "{}",
        Solution::length_of_longest_substring("bbbbb".to_string())
    );
    println!(
        "{}",
        Solution::length_of_longest_substring("pwwkew".to_string())
    );
}

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut win_begin = 0;
        let mut win_end = 0;
        let mut freq = vec![0; u8::MAX as usize + 1];
        let mut ans = 0;
        let mut count = 0;
        while win_end < n {
            let b = s[win_end] as usize;
            freq[b] += 1;
            if freq[b] == 1 {
                count += 1;
                ans = ans.max(count);
            } else {
                loop {
                    let c = s[win_begin] as usize;
                    freq[c] -= 1;
                    win_begin += 1;
                    if c == b {
                        break;
                    }
                    count -= 1;
                }
            }
            win_end += 1;
        }
        ans
    }
}
