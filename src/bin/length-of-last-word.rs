fn main() {
    println!(
        "{}",
        Solution::length_of_last_word(String::from("Hello World"))
    );
    println!("{}", Solution::length_of_last_word(String::from("World")));
}

struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let bs = s.as_bytes();
        let mut word_start = false;
        let mut length = 0;
        for i in (0..bs.len()).rev() {
            let b = bs[i];
            if !word_start {
                if b == b' ' {
                    continue;
                } else {
                    length += 1;
                    word_start = true;
                }
            } else {
                if b == b' ' {
                    break;
                } else {
                    length += 1;
                }
            }
        }
        length
    }
}
