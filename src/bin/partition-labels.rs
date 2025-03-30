fn main() {
    println!(
        "{:?}",
        Solution::partition_labels("ababcbacadefegdehijhklij".to_string())
    );
}

struct Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let s = s.as_bytes();
        let mut ans = vec![];
        let mut freq = vec![0; 26];
        let mut letters = vec![false; 26];
        let mut letter_count = 0;
        for &b in s {
            freq[(b - b'a') as usize] += 1;
        }
        let mut prev = -1;
        for (i, &b) in s.iter().enumerate() {
            let idx = (b - b'a') as usize;
            if !letters[idx] {
                letter_count += 1;
                letters[idx] = true;
            }
            freq[idx] -= 1;
            if freq[idx] == 0 {
                letter_count -= 1;
            }
            if letter_count == 0 {
                ans.push(i as i32 - prev);
                prev = i as i32;
            }
        }
        ans
    }
}
