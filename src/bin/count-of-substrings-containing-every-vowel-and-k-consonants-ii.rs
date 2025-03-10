fn main() {
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
        help(word, k) - help(word, k + 1)
    }
}

#[inline]
fn help(word: &[u8], k: usize) -> i64 {
    let mut freq = vec![0; 6];
    let mut vowel_flags = 0;
    let mut left = 0;
    let mut ans = 0;
    for &b in word {
        let idx = letter_to_index(b);
        freq[idx] += 1;
        if idx < 5 {
            vowel_flags |= 1 << idx;
        }
        while vowel_flags == 0b11111 && freq[5] >= k {
            let d = word[left];
            let idx = letter_to_index(d);
            freq[idx] -= 1;
            if idx < 5 {
                if freq[idx] == 0 {
                    vowel_flags -= 1 << idx;
                }
            }
            left += 1;
        }
        ans += left as i64
    }
    ans
}

#[inline]
fn letter_to_index(b: u8) -> usize {
    return match b {
        b'a' => 0,
        b'e' => 1,
        b'i' => 2,
        b'o' => 3,
        b'u' => 4,
        _ => 5,
    };
}
