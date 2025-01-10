fn main() {
    println!(
        "{:?}",
        Solution::word_subsets(
            vec![
                "amazon".to_string(),
                "apple".to_string(),
                "facebook".to_string(),
                "google".to_string(),
                "leetcode".to_string(),
            ],
            vec!["e".to_string(), "o".to_string()],
        )
    );
}

struct Solution;

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut ans = Vec::with_capacity(words1.len());
        let mut word2_letter_freq = [0; 26];
        for w in words2.iter() {
            let freq = letter_freq(w.as_bytes());
            for i in 0..26 {
                word2_letter_freq[i] = word2_letter_freq[i].max(freq[i]);
            }
        }
        'out: for w in words1 {
            let freq = letter_freq(w.as_bytes());
            for i in 0..26 {
                if freq[i] < word2_letter_freq[i] {
                    continue 'out;
                }
            }
            ans.push(w);
        }
        ans
    }
}

#[inline(always)]
fn letter_freq(word: &[u8]) -> [i32; 26] {
    let mut freq = [0; 26];
    for &b in word {
        freq[(b - b'a') as usize] += 1;
    }
    freq
}
