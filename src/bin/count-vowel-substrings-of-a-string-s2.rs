fn main() {
    assert_eq!(2, Solution::count_vowel_substrings("aeiouu".to_string()));
    assert_eq!(
        7,
        Solution::count_vowel_substrings("cuaieuouac".to_string())
    );
}

struct Solution;

impl Solution {
    pub fn count_vowel_substrings(word: String) -> i32 {
        let word = word.as_bytes();
        const VOWEL_MASK: usize = 1065233;
        let mut vowel_flags: usize = 0;
        let mut freq = vec![0; 26];
        let mut offset = 0;
        let mut left = 0;
        let mut ans = 0;
        for (j, &b) in word.iter().enumerate() {
            let i = (b - b'a') as usize;
            freq[i] += 1;
            if (1 << i) & VOWEL_MASK > 0 {
                vowel_flags |= 1 << i;
                while vowel_flags == VOWEL_MASK {
                    let d = word[left];
                    let i = (d - b'a') as usize;
                    freq[i] -= 1;
                    if (1 << i) & VOWEL_MASK > 0 {
                        if freq[i] == 0 {
                            vowel_flags -= 1 << i;
                        }
                    }
                    left += 1;
                }
                ans += (left - offset) as i32
            } else {
                offset = j + 1;
                left = offset;
                vowel_flags = 0;
                freq.fill(0);
            }
        }
        ans
    }
}
