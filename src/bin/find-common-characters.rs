fn main() {
    println!(
        "{:?}",
        Solution::common_chars(vec![
            "bella".to_string(),
            "label".to_string(),
            "roller".to_string()
        ])
    );
}

struct Solution;

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let n: usize = 26;
        let m = words.len();
        let mut freqs: Vec<Vec<usize>> = vec![vec![0; n]; words.len()];
        for (i, x) in words.iter().enumerate() {
            for b in x.bytes() {
                freqs[i][(b - b'a') as usize] += 1;
            }
        }
        let mut ans: Vec<String> = vec![];
        for i in 0..n {
            let mut min = 1001;
            for j in 0..m {
                min = min.min(freqs[j][i])
            }
            for _ in 0..min {
                ans.push(char::from(i as u8 + b'a').to_string());
            }
        }
        ans
    }
}
