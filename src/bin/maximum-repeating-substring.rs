fn main() {
    println!(
        "{}",
        Solution::max_repeating("ababc".to_string(), "ab".to_string())
    );
}

struct Solution;

impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let mut ans = 0;
        let mut k = 1;

        while sequence.contains(&word.repeat(k as usize)) {
            ans = k;
            k += 1;
        }

        ans
    }
}
