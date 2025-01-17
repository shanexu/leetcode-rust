fn main() {
    println!(
        "{}",
        Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string())
    );
}

struct Solution;
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut freq_t = [0; u8::MAX as usize + 1];
        let mut freq_s = [0; u8::MAX as usize + 1];
        let t = t.as_bytes();
        for &b in t {
            freq_t[b as usize] += 1;
        }
        let s = s.as_bytes();
        let mut win_begin = 0;
        let mut win_end = 0;
        let n = s.len();
        let m = t.len();
        let mut count = 0;
        let mut ans_len = n + 1;
        let mut ans: &[u8] = &[];
        while win_end < n {
            let b = s[win_end];
            let bi = b as usize;
            freq_s[bi] += 1;
            if freq_t[bi] != 0 && freq_s[bi] <= freq_t[bi] {
                count += 1;
            }
            while count == m {
                if win_end - win_begin + 1 < ans_len {
                    ans_len = win_end - win_begin + 1;
                    ans = &s[win_begin..win_end + 1];
                }
                let c = s[win_begin];
                let ci = c as usize;
                freq_s[ci] -= 1;
                if freq_t[ci] != 0 && freq_s[ci] < freq_t[ci] {
                    count -= 1;
                }
                win_begin += 1;
            }
            win_end += 1;
        }
        String::from_utf8(ans.to_vec()).unwrap()
    }
}
