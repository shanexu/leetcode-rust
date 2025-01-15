fn main() {
    println!(
        "{}",
        Solution::most_common_word(
            "Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
            vec!["hit".to_string()],
        )
    );
    println!("{}", Solution::most_common_word("Bob".to_string(), vec![],));
}

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let paragraph = paragraph.to_ascii_lowercase();
        let paragraph = paragraph.as_bytes();
        let mut banned_words = std::collections::HashSet::new();
        for w in banned.iter() {
            banned_words.insert(w.as_bytes());
        }
        let mut word_start = 0usize;
        let n = paragraph.len();
        let mut word_map: HashMap<&[u8], i32> = HashMap::new();
        let mut max_word: &[u8] = &[];
        let mut max_count = 0;
        for i in 0..n {
            let b = paragraph[i];
            match b {
                b' ' | b'!' | b'?' | b'\'' | b',' | b';' | b'.' => {
                    let word = &paragraph[word_start..i];
                    if word.len() > 0 && !banned_words.contains(&word) {
                        let count = word_map.entry(word).or_insert(0);
                        *count += 1;
                        if *count > max_count {
                            max_count = *count;
                            max_word = word;
                        }
                    }
                    word_start = i + 1;
                }
                _ => (),
            }
        }
        {
            let word = &paragraph[word_start..];
            if word.len() > 0 && !banned_words.contains(&word) {
                let count = word_map.entry(word).or_insert(0);
                *count += 1;
                if *count > max_count {
                    max_count = *count;
                    max_word = word;
                }
            }
        }
        String::from_utf8_lossy(max_word).to_string()
    }
}
