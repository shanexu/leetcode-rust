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
        count_at_least_k(word, 0) - count_at_least_k(word, 1)
    }
}

#[inline(always)]
fn count_at_least_k(word: &[u8], k: usize) -> i32 {
    const VOWEL_MASK: usize = 1065233;
    let mut vowel_flags: usize = 0;
    let mut freq = vec![0; 26];
    let mut left = 0;
    let mut ans = 0;
    let mut consonants = 0;
    for &b in word {
        let i = (b - b'a') as usize;
        freq[i] += 1;
        if (1 << i) & VOWEL_MASK > 0 {
            vowel_flags |= 1 << i;
        } else {
            consonants += 1;
        }
        while vowel_flags == VOWEL_MASK && consonants >= k {
            let d = word[left];
            let i = (d - b'a') as usize;
            freq[i] -= 1;
            if (1 << i) & VOWEL_MASK > 0 {
                if freq[i] == 0 {
                    vowel_flags -= 1 << i;
                }
            } else {
                consonants -= 1;
            }
            left += 1;
        }
        ans += left as i32
    }
    ans
}
