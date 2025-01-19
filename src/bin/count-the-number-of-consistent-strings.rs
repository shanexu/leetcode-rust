fn main() {}

struct Solution;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut freq = 0u32;
        let mut allowed = allowed.as_bytes();
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