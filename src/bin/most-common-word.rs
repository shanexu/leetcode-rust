use leetcode_rust::vec_string;

fn main() {
    println!(
        "{}",
        Solution::most_common_word(
            "Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
            vec_string!["hit"],
        )
    );
    println!("{}", Solution::most_common_word("Bob".to_string(), vec![],));
}

struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let paragraph = paragraph.to_ascii_lowercase();
        let mut banned_words: HashSet<String> = HashSet::new();

        for w in banned.iter() {
            banned_words.insert(w.clone()); // Store banned words as Strings
        }

        let mut word_map: HashMap<String, i32> = HashMap::new();
        let mut max_word = String::new();
        let mut max_count = 0;

        // Split the paragraph into words
        let words = paragraph.split(|c: char| !c.is_alphanumeric());

        for word in words {
            let word = word.trim(); // Trim whitespace
            if !word.is_empty() && !banned_words.contains(word) {
                let count = word_map.entry(word.to_string()).or_insert(0);
                *count += 1;

                // Only update max_word and max_count if the current count exceeds the max_count
                if *count > max_count {
                    max_count = *count;
                    max_word = word.to_string();
                }
            }
        }

        max_word
    }
}
