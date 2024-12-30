use std::collections::HashMap;

fn main() {
    println!(
        "{}",
        Solution::length_of_longest_substring(String::from("abcabcbb"))
    );
    println!(
        "{}",
        Solution::length_of_longest_substring(String::from("bbbbb"))
    );
    println!(
        "{}",
        Solution::length_of_longest_substring(String::from("pwwkew"))
    );
    println!(
        "{}",
        Solution::length_of_longest_substring(String::from(""))
    );
    println!(
        "{}",
        Solution::length_of_longest_substring(String::from("abba"))
    );
}

struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map: HashMap<u8, usize> = HashMap::new();
        let mut max_length: i32 = 0;
        let mut current_length: i32 = 0;
        let mut begin_idx = 0;
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            let old_value = map.insert(item, i);
            match old_value {
                None => current_length += 1,
                Some(v) => {
                    if v >= begin_idx {
                        current_length = (i - v) as i32;
                        begin_idx = v + 1;
                    } else {
                        current_length += 1;
                    }
                }
            };
            if current_length > max_length {
                max_length = current_length;
            }
        }
        max_length
    }
}
