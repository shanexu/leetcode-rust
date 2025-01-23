fn main() {
    println!(
        "{}",
        Solution::ladder_length(
            "hit".to_string(),
            "cog".to_string(),
            vec![
                "hot".to_string(),
                "dot".to_string(),
                "dog".to_string(),
                "lot".to_string(),
                "log".to_string(),
                "cog".to_string()
            ]
        )
    );
}

struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        if begin_word == end_word {
            return 0;
        }
        let mut ans = 0;
        let word_length = word_list[0].as_bytes().len();
        let mut word_dict = std::collections::HashSet::new();
        for word in word_list {
            word_dict.insert(word);
        }
        if !word_dict.contains(&end_word) {
            return 0;
        }
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(begin_word);
        while !queue.is_empty() {
            ans += 1;
            let q_size = queue.len();
            for _ in 0..q_size {
                let word = queue.pop_front().unwrap();
                let mut word = word.as_bytes().to_vec();
                for k in 0..word_length {
                    let orig = word[k];
                    for c in b'a'..=b'z' {
                        if c == orig {
                            continue;
                        }
                        word[k] = c;
                        if word == end_word.as_bytes() {
                            return ans + 1;
                        }
                        let new_word = String::from_utf8(word.clone()).unwrap();
                        if !word_dict.contains(&new_word) {
                            continue;
                        }
                        word_dict.remove(&new_word);
                        queue.push_back(new_word);
                    }
                    word[k] = orig;
                }
            }
        }
        0
    }
}
