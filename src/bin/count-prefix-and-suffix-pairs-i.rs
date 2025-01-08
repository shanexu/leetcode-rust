fn main() {
    println!(
        "{:?}",
        Solution::count_prefix_suffix_pairs(vec![
            "abcd".to_string(),
            "xyabcdyx".to_string(),
            "abcd".to_string()
        ])
    );
}

struct Solution;

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let n = words.len();
        let mut ans = 0;
        for i in 0..n {
            for j in i + 1..n {
                let a = &words[i];
                let b = &words[j];
                if b.starts_with(a) && b.ends_with(a) {
                    ans += 1;
                }
            }
        }
        ans
    }
}
