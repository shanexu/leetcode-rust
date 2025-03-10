fn main() {
    let mut vowel_mask: usize = 0;
    for &b in "aeiou".as_bytes() {
        vowel_mask |= 1 << (b as u32 - b'a' as u32);
    }
    println!("{}", vowel_mask);
    assert_eq!(0, Solution::count_of_substrings("aeioqq".to_string(), 1));
    assert_eq!(1, Solution::count_of_substrings("aeiou".to_string(), 0));
    assert_eq!(
        3,
        Solution::count_of_substrings("ieaouqqieaouqq".to_string(), 1)
    );
    assert_eq!(1, Solution::count_of_substrings("buoeia".to_string(), 0));
    assert_eq!(3, Solution::count_of_substrings("iqeaouqi".to_string(), 2));
    assert_eq!(1, Solution::count_of_substrings("douieua".to_string(), 1));
    assert_eq!(4, Solution::count_of_substrings("aoaiuefi".to_string(), 1));
}

struct Solution;

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        let word = word.as_bytes();
        let k = k as usize;
        count_at_least_k(word, k) - count_at_least_k(word, k + 1)
    }
}

#[inline(always)]
fn count_at_least_k(word: &[u8], k: usize) -> i64 {
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
        ans += left as i64
    }
    ans
}
