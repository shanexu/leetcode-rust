fn main() {
    println!(
        "{:?}",
        Solution::partition_labels("ababcbacadefegdehijhklij".to_string())
    );
}

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let s = s.as_bytes();
        let mut ans = vec![];
        let mut freq = vec![0; 26];
        let mut letters: HashSet<u8> = HashSet::new();
        for &b in s {
            freq[(b - b'a') as usize] += 1;
        }
        let mut prev = -1;
        for (i, &b) in s.iter().enumerate() {
            letters.insert(b);
            freq[(b - b'a') as usize] -= 1;
            if freq[(b - b'a') as usize] == 0 {
                letters.remove(&b);
            }
            if letters.len() == 0 {
                ans.push(i as i32 - prev);
                prev = i as i32;
            }
        }
        ans
    }
}
