fn main() {}

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut words_count: HashMap<&str, i32> = HashMap::new();

        // Count words in s1
        for word in s1.split_whitespace() {
            *words_count.entry(word).or_insert(0) += 1;
        }

        // Count words in s2
        for word in s2.split_whitespace() {
            *words_count.entry(word).or_insert(0) += 1;
        }

        let uncommon_words: Vec<String> = words_count
            .iter()
            .filter_map(|(&word, &count)| {
                if count == 1 {
                    Some(word.to_string())
                } else {
                    None
                }
            })
            .collect();

        uncommon_words
    }
}
