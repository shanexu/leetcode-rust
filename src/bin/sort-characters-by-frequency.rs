fn main() {}

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let s = s.as_bytes();
        let mut freq = HashMap::new();
        for &b in s {
            freq.entry(b).and_modify(|c| *c += 1).or_insert(1);
        }
        let mut vec: Vec<(i32, u8)> = freq.into_iter().map(|(b, c)| (c, b)).collect();
        vec.sort();
        let mut ans = Vec::with_capacity(s.len());
        for (f, b) in vec.iter().rev() {
            for _ in 0..*f {
                ans.push(*b);
            }
        }
        String::from_utf8(ans).unwrap()
    }
}
