fn main() {
    assert_eq!(Solution::count_palindromic_subsequence("aabca".to_string()), 3);
    assert_eq!(Solution::count_palindromic_subsequence("adc".to_string()), 0);
    assert_eq!(Solution::count_palindromic_subsequence("bbcbaba".to_string()), 4);
    assert_eq!(Solution::count_palindromic_subsequence("cdcdcdcd".to_string()), 4);
}

struct Solution;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let s = s.as_bytes();
        let mut first = vec![s.len(); 26];
        let mut last = vec![0; 26];
        for (i, &b) in s.iter().enumerate() {
            let idx = (b - b'a') as usize;
            first[idx] = first[idx].min(i);
            last[idx] = i
        }
        let mut ans = 0i32;
        for i in 0..26 {
            if first[i] < last[i] {
                let mut bitmap = 0u32;
                for j in first[i] + 1..last[i] {
                    bitmap |= 1 << (s[j] - b'a') as u32
                }
                ans += bit_count(bitmap) as i32;
            }
        }
        ans
    }
}

fn bit_count(mut var0: u32) -> u32 {
    var0 -= var0 >> 1 & 1431655765;
    var0 = (var0 & 858993459) + (var0 >> 2 & 858993459);
    var0 = (var0 + (var0 >> 4)) & 252645135;
    var0 += var0 >> 8;
    var0 += var0 >> 16;
    var0 & 63
}
