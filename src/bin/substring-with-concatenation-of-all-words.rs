fn main() {
    println!(
        "{:?}",
        Solution::find_substring(
            String::from("barfoothefoobarman"),
            vec![String::from("foo"), String::from("bar")]
        )
    );

    println!(
        "{:?}",
        Solution::find_substring(
            String::from("wordgoodgoodgoodbestword"),
            vec![
                String::from("word"),
                String::from("good"),
                String::from("best"),
                String::from("word")
            ],
        )
    );

    println!(
        "{:?}",
        Solution::find_substring(
            String::from("barfoofoobarthefoobarman"),
            vec![
                String::from("bar"),
                String::from("foo"),
                String::from("the"),
            ]
        )
    )
}

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let word_length = words[0].len();
        let word_count = words.len();
        let total_length = word_count * word_length;
        let bs = s.as_bytes();
        let s_len = bs.len();
        if total_length > bs.len() {
            return vec![];
        }
        let mut values = vec![];
        let mut word_map: HashMap<&[u8], usize> = HashMap::new();
        for x in words.iter() {
            let count = word_map.entry(x.as_bytes()).or_insert(0);
            *count += 1;
        }

        for i in 0..word_length {
            let mut tmp_word_map = word_map.clone();
            let mut j = i;
            let mut count = word_count;
            while j <= s_len - total_length {
                let mut k = j;
                while k <= s_len - word_length {
                    let word = &bs[k..k + word_length];
                    if let Some(v) = tmp_word_map.get_mut(word) {
                        if *v == 0 {
                            loop {
                                let first_word = &bs[j..j + word_length];
                                j += word_length;
                                if first_word == word {
                                    k += word_length;
                                    break;
                                } else {
                                    *(tmp_word_map.get_mut(first_word).unwrap()) += 1;
                                    count += 1;
                                }
                            }
                        } else {
                            *v -= 1;
                            count -= 1;
                            k += word_length;
                            if count == 0 {
                                values.push(j as i32);
                                let first_word = &bs[j..j + word_length];
                                *(tmp_word_map.get_mut(first_word).unwrap()) += 1;
                                count += 1;
                                j += word_length;
                            }
                        }
                    } else {
                        j = k + word_length;
                        tmp_word_map = word_map.clone();
                        count = word_count;
                        break;
                    }
                }
            }
        }
        values
    }
}
