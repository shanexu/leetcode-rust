fn main() {
    println!("{}", Solution::reorganize_string("aab".to_string()));
}

struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let s = s.as_bytes();
        let n = s.len();
        let mut ans = Vec::with_capacity(n);
        let mut freq = [0; 26];
        for &b in s {
            freq[(b - b'a') as usize] += 1;
        }
        let mut heap = BinaryHeap::new();
        for (i, &c) in freq.iter().enumerate() {
            if c != 0 {
                heap.push((c, i as u8 + b'a'));
            }
        }
        while let Some((c, b)) = heap.pop() {
            if c == 0 {
                continue;
            }
            for _ in 1..c {
                ans.push(b);
                if let Some(mut peek) = heap.peek_mut() {
                    if peek.0 == 0 {
                        return "".to_string();
                    }
                    ans.push(peek.1);
                    peek.0 -= 1;
                } else {
                    return "".to_string();
                }
            }
            ans.push(b);
        }
        String::from_utf8(ans).unwrap()
    }
}
