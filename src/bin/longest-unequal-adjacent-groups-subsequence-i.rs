fn main() {
    println!(
        "{:?}",
        Solution::get_longest_subsequence(
            vec!["e".to_string(), "a".to_string(), "b".to_string()],
            vec![0, 0, 1]
        )
    );
    println!(
        "{:?}",
        Solution::get_longest_subsequence(
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string()
            ],
            vec![1, 0, 1, 1]
        )
    );
}

struct Solution;

impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut ans = vec![];
        let mut prev_group = groups[0];
        ans.push(words[0].clone());
        for (i, &g) in groups.iter().enumerate().skip(1) {
            if g == prev_group {
                continue;
            } else {
                prev_group = g;
                ans.push(words[i].clone());
            }
        }
        ans
    }
}
